// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Command-line interface for Materialize Cloud.

mod login;
mod regions;
mod shell;
mod profiles;
mod utils;

use serde::{Deserialize, Serialize};

use clap::{Args, ArgEnum, Parser, Subcommand};
use reqwest::{Client};

use crate::login::{login_with_browser, login_with_console};
use crate::profiles::{authenticate_profile, get_local_profile, validate_profile};
use crate::regions::{delete_region, enable_region, list_cloud_providers, list_regions, warning_delete_region};
use crate::shell::{shell};

#[derive(Debug, Clone, ArgEnum)]
enum CloudProviderRegion {
    usEast_1,
    euWest_1
}

/// Materialize CLI
#[derive(Debug, Parser)]
#[clap(name = "mz")]
#[clap(about = "Materialize CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Login to Materialize
    Login(Login),
    /// Enable or delete a region
    Regions(Regions),
    /// Shell
    Shell
}

#[derive(Debug, Args)]
struct Login {
    #[clap(subcommand)]
    command: Option<LoginCommands>
}

#[derive(Debug, Subcommand)]
enum LoginCommands {
    Interactive
}

#[derive(Debug, Args)]
struct Regions {
    #[clap(subcommand)]
    command: RegionsCommands
}

#[derive(Debug, Subcommand)]
enum RegionsCommands {
    Enable {
        #[clap(arg_enum)]
        cloud_provider_region: CloudProviderRegion,
    },
    Delete {
        #[clap(arg_enum)]
        cloud_provider_region: CloudProviderRegion,
    },
    List
}

/**
 ** Internal types, strucs and enums
 **/

#[derive(Debug, Deserialize)]
struct Region {
    coordd_pgwire_address: String,
    coordd_https_address: String
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CloudProvider {
    provider: String,
    region: String,
    environment_controller_url: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FronteggAuthUser {
    mfa_required: bool,
    access_token: String,
    refresh_token: String,
    expires_in: u16,
    expires: String,
    email_verified: bool
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct FronteggAuthMachine {
    access_token: String,
    refresh_token: String,
    expires_in: u16,
    expires: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FronteggAPIToken {
    client_id: String,
    description: String,
    created_at: String,
    secret: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowserAPIToken {
    client_id: String,
    description: String,
    secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Profile {
    client_id: String,
    secret: String,
    default_region: Option<String>
}

const PROFILES_DIR_NAME: &str = "mz";
const PROFILES_FILE_NAME: &str = "profiles.toml";
const CLOUD_PROVIDERS_URL: &str = "https://staging.cloud.materialize.com/api/cloud-providers";
const API_TOKEN_AUTH_URL: &str = "https://materialize-staging.frontegg.com/identity/resources/users/api-tokens/v1";
const USER_AUTH_URL: &str = "https://materialize-staging.frontegg.com/frontegg/identity/resources/auth/v1/user";
const MACHINE_AUTH_URL: &str = "https://admin.staging.cloud.materialize.com/identity/resources/auth/v1/api-token";

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {

        Commands::Login(login_cmd) => {
            match login_cmd.command {
                Some(LoginCommands::Interactive) => {
                    match get_local_profile() {
                        None => login_with_console().await.unwrap(),
                        Some(profile) => println!("There is a profile available: {:?} \nPlease, remove it to continue.", profile)
                    }
                }
                _ => {
                    match get_local_profile() {
                        None => login_with_browser().await.unwrap(),
                        Some(profile) => println!("There is a profile available: {:?} \nPlease, remove it to continue.", profile)
                    }
                }
            }
        }

        Commands::Regions(regions_cmd) => {
            let client = Client::new();

            match regions_cmd.command {
                RegionsCommands::Enable { cloud_provider_region } => {
                    println!("Enabling cloud provider region.");

                    match validate_profile(client.clone()).await {
                        Some(frontegg_auth_machine) => {
                            match enable_region(client, cloud_provider_region, frontegg_auth_machine).await {
                                 Ok(region) => {
                                     println!("Region enabled: {:?}", region);
                                 }
                                Err(e) => println!("Error enabling region: {:?}", e),
                            }
                        }
                        None => {}
                    }
                }
                RegionsCommands::Delete { cloud_provider_region } => {
                    println!("Delete cloud provider region.");

                    if warning_delete_region(cloud_provider_region.clone()).await {
                        match validate_profile(client.clone()).await {
                            Some(frontegg_auth_machine) => {
                                println!("Deleting region. The operation may take a couple of minutes.");

                                match delete_region(client.clone(), cloud_provider_region, frontegg_auth_machine).await {
                                    Ok(_r) => {
                                        println!("Region deleted.")
                                    }
                                    Err(e) => println!("Error deleting region: {:?}", e)
                                }
                            }
                            None => {}
                        }
                    }
                }
                RegionsCommands::List => {
                    match validate_profile(client.clone()).await {
                        Some(frontegg_auth_machine) => {
                            println!("Listing regions.");
                            match list_cloud_providers(client.clone(), frontegg_auth_machine.clone()).await {
                                Ok(cloud_providers) => {
                                    let regions = list_regions(
                                        cloud_providers,
                                        client.clone(),
                                        frontegg_auth_machine
                                    ).await;
                                    println!("Regions: {:?}", regions);
                                },
                                Err(error) => panic!("Error retrieving cloud providers: {:?}", error)
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        Commands::Shell => {
            // TODO: Use local profile to retrieve default region.
            match get_local_profile() {
                Some(profile) => {
                    let client = Client::new();
                    match authenticate_profile(client.clone(), profile.clone()).await {
                        Ok(frontegg_auth_machine) => {
                            match shell(client, profile, frontegg_auth_machine).await {
                                _ => println!("Finish.")
                            }
                        },
                        Err(error) => println!("Error authenticating profile : {:?}", error)
                    }
                }
                None => println!("Profile not found. Please, login using `mz login`.")
            };
        }
    }
}
