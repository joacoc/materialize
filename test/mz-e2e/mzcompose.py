# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

import argparse
import os
import ssl
import time
import urllib.parse

import pg8000

from materialize.mzcompose import (
    _wait_for_pg,
)
from materialize.mzcompose.composition import (
    Composition,
    WorkflowArgumentParser,
)
from materialize.mzcompose.services.cockroach import Cockroach
from materialize.mzcompose.services.materialized import Materialized
from materialize.mzcompose.services.mz import Mz
from materialize.mzcompose.services.testdrive import Testdrive
from materialize.ui import UIError

REGION = "aws/us-east-1"
ENVIRONMENT = os.getenv("ENVIRONMENT", "staging")
USERNAME = os.getenv("NIGHTLY_CANARY_USERNAME", "infra+bot@materialize.com")
APP_PASSWORD = os.environ["MZ_CLI_APP_PASSWORD"]
VERSION = "devel-" + os.environ["BUILDKITE_COMMIT"]

# The DevEx account in the Confluent Cloud is used to provide Kafka services
KAFKA_BOOTSTRAP_SERVER = "pkc-n00kk.us-east-1.aws.confluent.cloud:9092"
SCHEMA_REGISTRY_ENDPOINT = "https://psrc-8kz20.us-east-2.aws.confluent.cloud"
# The actual values are stored in the i2 repository
CONFLUENT_API_KEY = os.environ["CONFLUENT_CLOUD_DEVEX_KAFKA_USERNAME"]
CONFLUENT_API_SECRET = os.environ["CONFLUENT_CLOUD_DEVEX_KAFKA_PASSWORD"]

SERVICES = [
    Cockroach(setup_materialize=True),
    Materialized(
        # We use materialize/environmentd and not materialize/materialized here
        # in order to ensure a perfect match to the container that should be
        # deployed to the cloud
        image=f"materialize/environmentd:{VERSION}",
        external_cockroach=True,
        persist_blob_url="file:///mzdata/persist/blob",
        options=[
            "--orchestrator-process-secrets-directory=/mzdata/secrets",
            "--orchestrator-process-scratch-directory=/scratch",
        ],
    ),
    Testdrive(),  # Overriden below
    Mz(
        region=REGION,
        environment=ENVIRONMENT,
        app_password=APP_PASSWORD,
    ),
]


def workflow_default(c: Composition, parser: WorkflowArgumentParser) -> None:
    """Deploy the current source to the cloud and run tests."""

    parser.add_argument(
        "--cleanup",
        default=True,
        action=argparse.BooleanOptionalAction,
        help="Destroy the region at the end of the workflow.",
    )
    parser.add_argument(
        "--version-check",
        default=True,
        action=argparse.BooleanOptionalAction,
        help="Perform a version check.",
    )

    parser.add_argument(
        "td_files",
        nargs="*",
        default=["*.td"],
        help="run against the specified files",
    )

    args = parser.parse_args()

    if args.cleanup:
        workflow_disable_region(c)

    test_failed = True
    try:
        print(f"Enabling region using Mz version {VERSION} ...")
        try:
            c.run("mz", "region", "enable", "--version", VERSION)
        except UIError:
            # Work around https://github.com/MaterializeInc/materialize/issues/17219
            pass

        time.sleep(10)

        assert "materialize.cloud" in cloud_hostname(c)
        wait_for_cloud(c)

        if args.version_check:
            version_check(c)

        # Test - `mz app-password`
        # Assert `mz app-password create`
        new_app_password_name = "Materialize CLI (mz) - Nightlies"
        output = c.run("mz", "app-password", "create", new_app_password_name, capture=True)
        assert "mzp_" in output.stdout

        # Assert `mz app-password list`
        output = c.run("mz", "app-password", "list", capture=True)
        assert new_app_password_name in output.stdout

        # // Test - `mz secrets`
        secret = "decode('c2VjcmV0Cg==', 'base64')"
        output = c.run("mz", "secret", "create", "CI_SECRET", stdin=secret, capture=True)
        assert output.returncode == 0

        output = c.run("mz", "sql", "--", "-q", "-c", "\"SHOW SECRETS;\"", capture=True)
        assert "CI_SECRET" in output.stdout

        # Assert using force
        output = c.run("mz", "secret", "create", "CI_SECRET", "force", stdin=secret, capture=True)
        assert output.returncode == 0

        # Test - `mz user`
        username = "mz_ci_e2e_test_nightlies@materialize.com"

        # Try to remove the username if it exist before trying to create one.
        output = c.run("mz", "user", "remove", username, capture=True)

        output = c.run("mz", "user", "create", username, "MZ_CI", capture=True)
        assert output.returncode == 0

        output = c.run("mz", "user", "list")
        assert output.returncode == 0
        assert username in output.stdout

        output = c.run("mz", "user", "remove", username, capture=True)
        assert output.returncode == 0

        output = c.run("mz", "user", "list", capture=True)
        assert username not in output.stdout

        # Test - `mz region list`
        # Enable, disable and show are already tested.
        output = c.run("mz", "region", "list", capture=True)
        us_east_1_line = output.stdout.split("\n")[3]
        assert "aws/us-east-1" in us_east_1_line
        assert "enabled" in us_east_1_line

        # TODO: TEST using CSV and JSON.

        # TODO:
        # // Look for the '.psqlrc' file in the home dir.
        # let mut path = dirs::home_dir().expect("Error retrieving the home dir.");
        # path.push(".psqlrc-mz");

        # // Verify the content is ok
        # if Path::new(&path).exists() {
        #     let mut file = fs::File::open(&path).expect("Error opening the '.psqlrc-mz' file.");
        #     let mut content = String::new();
        #     file.read_to_string(&mut content)
        #         .expect("Error reading the '.psqlrc-mz' file.");

        #     if content != "\\timing\n\\include ~/.psqlrc" {
        #         panic!("Incorrect content in the '.psqlrc-mz' file.")
        #     }
        # } else {
        #     panic!("The configuration file '.psqlrc-mz', does not exists.")
        # }

        test_failed = False
    finally:
        # Clean up
        if args.cleanup:
            workflow_disable_region(c)

    assert not test_failed


def workflow_disable_region(c: Composition) -> None:
    print(f"Shutting down region {REGION} ...")

    c.run("mz", "region", "disable")


def cloud_hostname(c: Composition) -> str:
    print("Obtaining hostname of cloud instance ...")
    region_status = c.run("mz", "region", "show", capture=True)
    sql_line = region_status.stdout.split("\n")[2]
    cloud_url = sql_line.split("\t")[1].strip()
    # It is necessary to append the 'https://' protocol; otherwise, urllib can't parse it correctly.
    cloud_hostname = urllib.parse.urlparse("https://" + cloud_url).hostname
    return str(cloud_hostname)


def wait_for_cloud(c: Composition) -> None:
    print(f"Waiting for cloud cluster to come up with username {USERNAME} ...")
    _wait_for_pg(
        host=cloud_hostname(c),
        user=USERNAME,
        password=APP_PASSWORD,
        port=6875,
        query="SELECT 1",
        expected=[[1]],
        timeout_secs=900,
        dbname="materialize",
        ssl_context=ssl.SSLContext(),
        # print_result=True
    )


def version_check(c: Composition) -> None:
    print("Obtaining mz_version() string from local instance ...")
    c.up("materialized")
    local_version = c.sql_query("SELECT mz_version();")[0][0]

    print("Obtaining mz_version() string from the cloud ...")
    cloud_cursor = pg8000.connect(
        host=cloud_hostname(c),
        user=USERNAME,
        password=APP_PASSWORD,
        port=6875,
        ssl_context=ssl.SSLContext(),
    ).cursor()
    cloud_cursor.execute("SELECT mz_version()")
    result = cloud_cursor.fetchone()
    assert result is not None
    cloud_version = result[0]
    assert (
        local_version == cloud_version
    ), f"local version: {local_version} is not identical to cloud version: {cloud_version}"
