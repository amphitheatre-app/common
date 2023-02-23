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

use serde::Serialize;

/// AuthConfig contains authorization information for connecting to a Registry
/// Inlined what we use from github.com/docker/cli/cli/config/types
#[derive(Serialize)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth: Option<String>,
}

/// DockerConfig ~/.docker/config.json file info
#[derive(Serialize)]
pub struct DockerConfig {
    pub auths: Option<HashMap<String, AuthConfig>>,
}
