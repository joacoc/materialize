use std::path::PathBuf;
use std::fs;
use std::{collections::HashMap};
use dirs::config_dir;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, Error};
use crate::{FronteggAuthMachine, MACHINE_AUTH_URL, Profile, PROFILES_DIR_NAME, PROFILES_FILE_NAME};

/// ----------------------------
///  Profiles handling
/// ----------------------------

fn get_config_path() -> PathBuf {
    match config_dir() {
        Some(path) => path,
        None => panic!("Problem trying to find the config dir.")
    }
}

pub(crate) async fn authenticate_profile(client: Client, profile: Profile) -> Result<FronteggAuthMachine, Error> {
    println!("Authenticating profile in Frontegg");

    let mut access_token_request_body = HashMap::new();
    access_token_request_body.insert("clientId", profile.client_id);
    access_token_request_body.insert("secret", profile.secret);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    client.post(MACHINE_AUTH_URL)
        .headers(headers)
        .json(&access_token_request_body)
        .send()
        .await?
        .json::<FronteggAuthMachine>()
        .await
}

fn create_profile_dir_if_not_exists() {
    let mut config_path = get_config_path();
    config_path.push(PROFILES_DIR_NAME);

    // Check if path exists
    if fs::metadata(config_path.clone()).is_err() {
        println!("Creating dir!");

        fs::create_dir_all(config_path.as_path()).unwrap();
    };
}

fn write_profile(profile: Profile) -> std::io::Result<()> {
    let mut config_path = get_config_path();
    config_path.push(PROFILES_DIR_NAME);
    config_path.push(PROFILES_FILE_NAME);

    let toml = toml::to_string(&profile).unwrap();
    fs::write(config_path, toml)
}

pub(crate) fn save_profile(profile: Profile) -> std::io::Result<()> {
    println!("Creating profile.");
    create_profile_dir_if_not_exists();

    println!("Writing profile to file.");
    write_profile(profile)
}

pub(crate) fn get_local_profile() -> Option<Profile> {
    println!("Retrieving local profile");

    // Check if path exists
    create_profile_dir_if_not_exists();

    let mut config_path = get_config_path();
    config_path.push(PROFILES_DIR_NAME);
    config_path.push(PROFILES_FILE_NAME);

    println!("Path: {:?}", config_path.clone());

    // Check if profiles file exists
    if fs::metadata(config_path.clone()).is_err() {
        None
    } else {
        // Return profile
        match fs::read_to_string(config_path.as_path()) {
            Ok(profiles_serialized) => {
                match toml::from_str(&*profiles_serialized) {
                    Ok(profile) => Some(profile),
                    Err(error) => panic!("Problem parsing the profiles: {:?}", error)
                }
            }
            Err(error) => panic!("Problem opening the profiles file: {:?}", error)
        }
    }
}

pub(crate) async fn validate_profile(client: Client) -> Option<FronteggAuthMachine> {
    println!("Validating profile");

    match get_local_profile() {
        Some(profile) => {
            match authenticate_profile(client, profile).await {
                Ok(frontegg_auth_machine) => {
                    return Some(frontegg_auth_machine);
                }
                Err(error) => println!("Error authenticating profile : {:?}", error)
            }
        }
        None => println!("Profile not found. Please, login using `mz login`.")
    }

    None
}
