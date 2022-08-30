// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use crate::regions::{cloud_provider_region_details, list_cloud_providers, region_environment_details};
use crate::utils::{exit_with_fail_message, CloudProviderRegion};
use crate::{ExitMessage, FronteggAuthMachine, Profile, Environment};
use reqwest::Client;
use std::process::exit;
use subprocess::Exec;

/// ----------------------------
/// Shell command
/// ----------------------------

/// Parse host and port from the pgwire URL
pub(crate) fn parse_pgwire(envrionment: &Environment) -> (&str, &str) {
    let host = &envrionment.environmentd_pgwire_address[..envrionment.environmentd_pgwire_address.len() - 5];
    let port = &envrionment.environmentd_pgwire_address
        [envrionment.environmentd_pgwire_address.len() - 4..envrionment.environmentd_pgwire_address.len()];

    (host, port)
}

/// Runs psql as a subprocess command
fn run_psql_shell(profile: Profile, environment: &Environment) {
    let (host, port) = parse_pgwire(environment);
    let email = profile.email.clone();

    let output = Exec::cmd("psql")
        .arg("-U")
        .arg(email)
        .arg("-h")
        .arg(host)
        .arg("-p")
        .arg(port)
        .arg("materialize")
        .env("PGPASSWORD", password_from_profile(profile))
        .join()
        .expect("failed to execute process");

    assert!(output.success());
}

/// Runs pg_isready to check if an environment is healthy
pub(crate) fn check_environment_health(profile: Profile, environment: &Environment) -> bool {
    let (host, port) = parse_pgwire(environment);
    let email = profile.email.clone();

    let output = Exec::cmd("pg_isready")
        .arg("-U")
        .arg(email)
        .arg("-h")
        .arg(host)
        .arg("-p")
        .arg(port)
        .env("PGPASSWORD", password_from_profile(profile))
        .arg("-d")
        .arg("materialize")
        .arg("-q")
        .join()
        .unwrap();

    output.success()
}

/// Turn a profile into a Materialize cloud instance password
fn password_from_profile(profile: Profile) -> String {
    "mzp_".to_owned() + &profile.client_id + &profile.secret
}

/// Command to run a shell (psql) on a Materialize cloud instance
pub(crate) async fn shell(
    client: Client,
    profile: Profile,
    frontegg_auth_machine: FronteggAuthMachine,
    cloud_provider_region: CloudProviderRegion,
) {
    match list_cloud_providers(&client, &frontegg_auth_machine).await {
        Ok(cloud_providers) => {
            let region = cloud_provider_region;

            // TODO: A map would be more efficient.
            let selected_cloud_provider_filtered = cloud_providers
                .into_iter()
                .find(|cloud_provider| cloud_provider.region == region.region_name());

            match selected_cloud_provider_filtered {
                Some(cloud_provider) => {
                    match cloud_provider_region_details(
                        &client,
                        &cloud_provider,
                        &frontegg_auth_machine,
                    )
                    .await
                    {
                        Ok(Some(mut cloud_provider_regions)) => {
                            println!("WOHOOO1");
                            println!("{:?}", cloud_provider_regions);
                            match cloud_provider_regions.pop() {
                                Some(region) => {
                                    // TODO: Replicated code.
                                                        println!("WOHOOO");
                                    match region_environment_details(&client, &region, &frontegg_auth_machine).await {
                                        Ok(environment_details) => {
                                            if let Some(mut environment_list) = environment_details {
                                                match environment_list.pop() {
                                                    Some(environment) => {
                                                        run_psql_shell(profile, &environment)
                                                    },
                                                    None => exit_with_fail_message(ExitMessage::Str(
                                                        "Error. Missing environment.",
                                                    )),
                                                }
                                            } else {
                                                exit_with_fail_message(ExitMessage::Str(
                                                    "Environment unavailable.",
                                                ));
                                            }
                                        },
                                        Err(error) => exit_with_fail_message(ExitMessage::String(
                                            format!("Error getting environment details: {:}", error),
                                        )),
                                    }
                                }
                                None => {
                                    println!("The region is not enabled.");
                                    exit(0);
                                }
                            }
                        }
                        Err(error) => exit_with_fail_message(ExitMessage::String(format!(
                            "Error retrieving region details: {:?}",
                            error
                        ))),
                        Ok(None) => {println!("No region found.")}
                    }
                }
                None => exit_with_fail_message(ExitMessage::Str("Unknown region.")),
            }
        }
        Err(error) => exit_with_fail_message(ExitMessage::String(format!(
            "Error retrieving cloud providers: {:?}",
            error
        ))),
    }
}
