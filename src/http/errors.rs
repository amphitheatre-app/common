// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

/// Represents the possible errors thrown while interacting with the Amphitheatre API
#[derive(Error, Debug)]
pub enum HTTPError {
    #[error("Deserialization Error {0}")]
    Deserialization(#[source] serde_json::Error),

    #[error("Invalid Header Value {0}")]
    InvalidHeaderValue(InvalidHeaderValue),

    #[error("Reqwest Error {0}")]
    ReqwestError(#[source] reqwest::Error),

    #[error("Url Parse Error {0}")]
    UrlParse(#[source] url::ParseError),
}
