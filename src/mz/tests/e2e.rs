// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

// BEGIN LINT CONFIG
// DO NOT EDIT. Automatically generated by bin/gen-lints.
// Have complaints about the noise? See the note in misc/python/materialize/cli/gen-lints.py first.
#![allow(clippy::style)]
#![allow(clippy::complexity)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::mutable_key_type)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::map_entry)]
#![allow(clippy::box_default)]
#![warn(clippy::bool_comparison)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::no_effect)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::zero_prefixed_literal)]
#![warn(clippy::borrowed_box)]
#![warn(clippy::deref_addrof)]
#![warn(clippy::double_must_use)]
#![warn(clippy::double_parens)]
#![warn(clippy::extra_unused_lifetimes)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_question_mark)]
#![warn(clippy::needless_return)]
#![warn(clippy::redundant_pattern)]
#![warn(clippy::redundant_slicing)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::single_component_path_imports)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::useless_asref)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::builtin_type_shadow)]
#![warn(clippy::duplicate_underscore_argument)]
#![warn(clippy::double_neg)]
#![warn(clippy::unnecessary_mut_passed)]
#![warn(clippy::wildcard_in_or_patterns)]
#![warn(clippy::crosspointer_transmute)]
#![warn(clippy::excessive_precision)]
#![warn(clippy::overflow_check_conditional)]
#![warn(clippy::as_conversions)]
#![warn(clippy::match_overlapping_arm)]
#![warn(clippy::zero_divided_by_zero)]
#![warn(clippy::must_use_unit)]
#![warn(clippy::suspicious_assignment_formatting)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_unary_op_formatting)]
#![warn(clippy::mut_mutex_lock)]
#![warn(clippy::print_literal)]
#![warn(clippy::same_item_push)]
#![warn(clippy::useless_format)]
#![warn(clippy::write_literal)]
#![warn(clippy::redundant_closure)]
#![warn(clippy::redundant_closure_call)]
#![warn(clippy::unnecessary_lazy_evaluations)]
#![warn(clippy::partialeq_ne_impl)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::transmutes_expressible_as_ptr_casts)]
#![warn(clippy::unused_async)]
#![warn(clippy::disallowed_methods)]
#![warn(clippy::disallowed_macros)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::from_over_into)]
// END LINT CONFIG

use std::{fs, time::Duration};

use assert_cmd::Command;
use mz::{config_file::ConfigFile, ui::OptionalStr};
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};
use uuid::{uuid, Uuid};

fn cmd() -> Command {
    let mut cmd = Command::cargo_bin("mz").unwrap();
    cmd.env_clear().timeout(Duration::from_secs(10));
    cmd
}

#[test]
#[cfg_attr(miri, ignore)] // unsupported operation: can't call foreign function `pipe2` on OS `linux`
fn test_version() {
    // We don't make assertions about the build SHA because caching in CI can
    // cause the test binary and `environmentd` to have different embedded SHAs.
    let expected_version = mz::BUILD_INFO.version;
    assert!(!expected_version.is_empty());
    cmd()
        .arg("-V")
        .assert()
        .success()
        .stdout(format!("mz {}\n", expected_version));
}

#[tokio::test]
#[cfg_attr(miri, ignore)] // unsupported operation: can't call foreign function `pipe2` on OS `linux`
async fn set_config_file() {
    let main_config_file = format!(
        r#"
            "profile" = "default"

            [profiles.default]
            app-password = "{}"
            region = "aws/us-east-1"
        "#,
        // TODO: Replace with CI PASSWORD
        std::env!("CI_PASSWORD")
    );
    fs::write(ConfigFile::default_path().unwrap(), main_config_file).unwrap();
}

#[test]
fn test_config_params() {
    #[derive(Deserialize, Serialize, Tabled)]
    pub struct ConfigParam<'a> {
        #[tabled(rename = "Name")]
        name: &'a str,
        #[tabled(rename = "Value")]
        value: OptionalStr<'a>,
    }

    let vec = vec![
        ConfigParam {
            name: "profile",
            value: mz::ui::OptionalStr(Some("default")),
        },
        ConfigParam {
            name: "vault",
            value: mz::ui::OptionalStr(Some("<unset>")),
        },
    ];
    let command_output = Table::new(vec).with(Style::psql()).to_string();

    let binding = cmd().arg("config").arg("list").assert().success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == command_output.trim());

    let binding = cmd()
        .arg("config")
        .arg("get")
        .arg("profile")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert!(stdout.trim() == "default");

    cmd()
        .arg("config")
        .arg("set")
        .arg("profile")
        .arg("random")
        .assert()
        .success();

    let binding = cmd()
        .arg("config")
        .arg("get")
        .arg("profile")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == "random");

    cmd()
        .arg("config")
        .arg("remove")
        .arg("profile")
        .assert()
        .success();

    let binding = cmd()
        .arg("config")
        .arg("get")
        .arg("profile")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == "default");
}

#[tokio::test]
async fn test_profile_commands() {
    #[derive(Deserialize, Serialize, Tabled)]
    pub struct ProfileConfigParam<'a> {
        #[tabled(rename = "Name")]
        name: &'a str,
        #[tabled(rename = "Value")]
        value: &'a str,
    }

    let vec = vec![
        ProfileConfigParam { name: "admin-endpoint", value: "<unset>" },
        ProfileConfigParam { name: "app-password", value: std::env!("CI_PASSWORD") },
        ProfileConfigParam { name: "cloud-endpoint", value: "<unset>" },
        ProfileConfigParam { name: "region", value: "aws/us-east-1" },
        ProfileConfigParam { name: "vault", value: "<unset>" },
    ];
    let command_output = Table::new(vec).with(Style::psql()).to_string();
    let binding = cmd()
        .arg("profile")
        .arg("config")
        .arg("list")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == command_output.trim());

    let binding = cmd()
        .arg("profile")
        .arg("config")
        .arg("get")
        .arg("region")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == "aws/us-east-1");

    cmd()
        .arg("profile")
        .arg("config")
        .arg("set")
        .arg("region")
        .arg("aws/eu-west-1")
        .assert()
        .success();

    let binding = cmd()
        .arg("profile")
        .arg("config")
        .arg("get")
        .arg("region")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == "aws/eu-west-1");

    // TODO: Remove an app-password. Breaks the CLI config. The same if the app-password is invalid.
    // TODO: Add more tests for config_set and config_remove when you implement the actual commands.
    // TODO: Profile init + Profile remove
}

#[tokio::test]
async fn test_app_password_commands() {
    #[derive(Deserialize, Serialize, Tabled)]
    pub struct AppPassword {
        #[tabled(rename = "Name")]
        description: String,
        #[tabled(rename = "Created At")]
        created_at: String,
    }

    let description = uuid::Uuid::new_v4();

    let binding = cmd()
        .arg("app-password")
        .arg("create")
        .arg(&description.to_string())
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.starts_with("mzp_"));

    let binding = cmd()
        .arg("app-password")
        .arg("list")
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.contains(&description.to_string()));
}

#[cfg(test)]
#[tokio::test]
async fn test_create_secret() {
    let description = format!("SAFE_TO_DELETE_{}", uuid::Uuid::new_v4());

    // Secrets
    cmd()
        .arg("secret")
        .arg("create")
        .arg(description.clone())
        .write_stdin("decode('c2VjcmV0Cg==', 'base64')")
        .assert()
        .success();

    // Force
    cmd()
        .arg("secret")
        .arg("create")
        .arg(description)
        .arg("force")
        .write_stdin("decode('c2VjcmV0Cg==', 'base64')")
        .assert()
        .success();
}

#[cfg(test)]
#[tokio::test]
async fn test_users() {
    let name = format!("SAFE_TO_DELETE_+{}", uuid::Uuid::new_v4());
    let email = format!("{}@materialize.com", name);

    // Users:
    // TODO: Check if the user exists first.
    cmd()
        .arg("user")
        .arg("create")
        .arg(email.clone())
        .arg(name)
        .assert()
        .success();

    let binding = cmd()
        .arg("user")
        .arg("list")
        .args(vec!["--format", "json"])
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.contains(&email.to_string()));

    cmd()
        .arg("user")
        .arg("remove")
        .arg(email.clone())
        .assert()
        .success();

    let binding = cmd()
        .arg("user")
        .arg("list")
        .args(vec!["--format", "json"])
        .assert()
        .success();

    let output = binding.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(!stdout.contains(&email.to_string()));
}

#[test]
fn test_regions() {
    #[derive(Deserialize, Serialize, Tabled)]
    pub struct Region<'a> {
        #[tabled(rename = "Region")]
        region: &'a str,
        #[tabled(rename = "Status")]
        status: &'a str,
    }

    let vec = vec![Region {region:"aws/eu-west-1", status: "enabled" }, Region {region:"aws/us-east-1", status: "enabled" }];
    let command_output = Table::new(vec).with(Style::psql()).to_string();
    let binding = cmd()
        .arg("region")
        .arg("list")
        .assert()
        .success();

    let output = binding
        .get_output();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert!(stdout.trim() == command_output.trim());

    let binding = cmd()
        .arg("region")
        .arg("show")
        .assert()
        .success();

    let output = binding
        .get_output();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", stdout.trim());
    assert!(stdout.trim().starts_with(&format!("Healthy: \t{}", "yes")))

    // TODO:
    // cmd()
    //     .arg("region")
    //     .arg("enable")
    //     .assert()
    //     .success();
}

// #[test]
// #[cfg_attr(miri, ignore)]
// fn test_e2e() {

//     // TODO: Add more interactions with Materialize
//     cmd().arg("sql").assert().success();

