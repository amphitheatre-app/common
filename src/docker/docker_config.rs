// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use data_encoding::BASE64;
use serde::Serialize;

use crate::config::{Credential, RegistryCredential};

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

impl DockerConfig {
    pub fn get_auth(&self, image_registry: &str) -> Option<&String> {
        if let Some(auths) = &self.auths {
            if let Some(credential) = auths.get(image_registry) {
                return credential.auth.as_ref();
            }

            let image_registry = normalize_registry(image_registry);
            if let Some((_, auth_str)) = auths
                .iter()
                .find(|(key, _)| normalize_key_to_registry(key) == image_registry)
            {
                return auth_str.auth.as_ref();
            }
        }

        None
    }
}

/// Normalizes a given key (image reference) into its resulting registry
fn normalize_key_to_registry(key: &str) -> &str {
    let stripped = key.strip_prefix("http://").unwrap_or(key);
    let mut stripped = key.strip_prefix("https://").unwrap_or(stripped);
    if stripped != key {
        stripped = stripped.split_once('/').unwrap_or((stripped, "")).0;
    }

    normalize_registry(stripped)
}

/// Converts the provided registry if a known `docker.io` host
/// is provided.
fn normalize_registry(registry: &str) -> &str {
    match registry {
        "registry-1.docker.io" | "docker.io" => "index.docker.io",
        _ => registry,
    }
}

/// Build a configuration that conforms to the `.dockerconfigjson` specification
impl From<&Vec<RegistryCredential>> for DockerConfig {
    fn from(entries: &Vec<RegistryCredential>) -> Self {
        let mut auths = HashMap::new();

        for credential in entries.iter() {
            let endpoint = &credential.server;
            let pair = format!("{}:{}", credential.username_any(), credential.password_any());
            let auth = BASE64.encode(pair.as_bytes());
            auths.insert(
                endpoint.clone(),
                AuthConfig {
                    username: Some(credential.username_any()),
                    password: Some(credential.password_any()),
                    auth: Some(auth),
                },
            );
        }

        DockerConfig { auths: Some(auths) }
    }
}
