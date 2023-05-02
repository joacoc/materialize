// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

//! This module implements Materialize cloud API methods
//! to GET, CREATE or DELETE an environment.
//! To delete an environment correctly make sure to
//! contact support.
//!
//! For a better experience retrieving all the available
//! environments, use [`Client::get_all_environments()`]

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::{cloud_provider::CloudProvider, region::Region, Client, Error};

/// An environment is represented in this structure
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    /// Represents the environmentd PG wire protocol address.
    ///
    /// Format: <ID>.<REGION>.<PROVIDER>.materialize.cloud:6875
    /// E.g.: 3es24sg5rghjku7josdcs5jd7.eu-west-1.aws.materialize.cloud:6875
    pub environmentd_pgwire_address: String,
    /// Represents the environmentd PG wire protocol address.
    ///
    /// Format: <ID>.<REGION>.<PROVIDER>.materialize.cloud:443
    /// E.g.: 3es24sg5rghjku7josdcs5jd7.eu-west-1.aws.materialize.cloud:443
    pub environmentd_https_address: String,
    /// Indicates true if the address is resolvable by DNS.
    pub resolvable: bool,
}

impl Client {
    /// Get an environment in a partciular region for the current user.
    pub async fn get_environment(&self, region: Region) -> Result<Environment, Error> {
        // Build subdomain:
        let subdomain = region.ec_subdomain();

        // Send request to the subdomain
        let req = self
            .build_request(Method::GET, ["api", "environment"], &subdomain)
            .await?;

        let environments: Vec<Environment> = self.send_request(req).await?;
        Ok(environments.get(0).unwrap().to_owned())
    }

    /// Get all the available environments for the current user.
    pub async fn get_all_environments(&self) -> Result<Vec<Environment>, Error> {
        let cloud_providers: Vec<CloudProvider> = self.list_cloud_providers().await?;
        let mut environments: Vec<Environment> = vec![];

        for cloud_provider in cloud_providers {
            let region = self.get_region(cloud_provider).await?;
            let environment = self.get_environment(region).await?;
            environments.push(environment);
        }

        Ok(environments)
    }

    /// Creates an environment in a particular region for the current user
    pub async fn create_environment(
        &self,
        version: Option<String>,
        environmentd_extra_args: Vec<String>,
        region: Region,
    ) -> Result<Region, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            #[serde(skip_serializing_if = "Option::is_none")]
            environmentd_image_ref: Option<String>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            environmentd_extra_args: Vec<String>,
        }

        let body = Body {
            environmentd_image_ref: version.map(|v| match v.split_once(':') {
                None => format!("materialize/environmentd:{v}"),
                Some((user, v)) => format!("{user}/environmentd:{v}"),
            }),
            environmentd_extra_args,
        };

        let subdomain = region.ec_subdomain();
        let req = self
            .build_request(Method::POST, ["api", "environmentassignment"], &subdomain)
            .await?;
        let req = req.json(&body);
        Ok(self.send_request(req).await?)
    }

    /// Deletes an environment in a particular region for the current user
    pub async fn delete_environment(&self, region: Region) -> Result<Region, Error> {
        let subdomain = region.ec_subdomain();
        let req = self
            .build_request(Method::DELETE, ["api", "environmentassignment"], &subdomain)
            .await?;
        Ok(self.send_request(req).await?)
    }
}
