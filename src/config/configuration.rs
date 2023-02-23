// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::credential::Credential;

/// Configuration is used to store user configurations on the client side,
/// such as Docker registry and SCM  credentials, and other propeaties
/// that need to be kept in sync with the server.
#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    /// Each cluster must have a Docker registry, here is its access credential
    pub registry: HashMap<String, Credential>,
    /// Access credentials for multiple code repositories will be matched based on endpoint.
    pub repositories: HashMap<String, Credential>,
}
