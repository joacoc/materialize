// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use reqwest::StatusCode;
use std::fmt;
use thiserror::Error;

/// An error returned by the Frontegg API.
#[derive(Debug, Clone)]
pub struct ApiError {
    /// The HTTP status code.
    pub status_code: StatusCode,
    /// A detailed message about the error conditions.
    pub messages: Vec<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (status {})",
            self.messages.join(","),
            self.status_code
        )
    }
}

impl std::error::Error for ApiError {}

#[derive(Error, Debug)]
pub enum CloudApiError {
    #[error("Network error during a Materialize cloud API request: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("API error during a Materialize cloud API request: {0}")]
    Api(#[from] ApiError),
    #[error("No environment available in this region.")]
    EmptyRegion,
}
