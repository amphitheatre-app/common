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

use serde::{Deserialize, Serialize};

use crate::utils::http::host;

/// `Credential` is used to provide common methods for accessing credentials.
pub trait Credential {
    /// Get the username of the credential
    fn username(&self) -> Option<String>;
    /// Get the password of the credential
    fn password(&self) -> Option<String>;
    /// Get the token of the credential
    fn token(&self) -> Option<String>;

    fn username_any(&self) -> String {
        self.username().unwrap_or_default()
    }

    fn password_any(&self) -> String {
        self.password().unwrap_or_default()
    }

    fn token_any(&self) -> String {
        self.token().unwrap_or_default()
    }

    /// Get the authentication scheme based on the credential
    fn scheme(&self) -> Scheme {
        if self.username().is_none() && self.token().is_some() {
            Scheme::Bearer
        } else if self.username().is_some() && self.password().is_some() {
            Scheme::Basic
        } else {
            Scheme::Unknown
        }
    }
}

/// `Credentials` is used to store access credentials on the client and cluster side,
/// such as Docker registry and SCM credentials, and other properties
/// that need to be kept in sync with the server.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Credentials {
    /// Access credentials for multiple docker registries.
    pub registries: Vec<RegistryCredential>,
    /// Access credentials for multiple code repositories.
    pub repositories: Option<Vec<RepositoryCredential>>,
}

impl Credentials {
    pub fn default_registry(&self) -> Option<&RegistryCredential> {
        self.registries.iter().find(|registry| registry.default)
    }

    /// Get the credential of the specified repository by repository server address.
    pub fn find_repository(&self, server: &str) -> Option<&RepositoryCredential> {
        let server = host(server);
        self.repositories.as_ref().and_then(|repositories| {
            repositories
                .iter()
                .find(|repository| server.is_some() && host(&repository.server) == server)
        })
    }
}

/// Access credentials for multiple docker registries.
/// indicating basic authentication when there is only a
/// username and password, otherwise bearer authentication.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RegistryCredential {
    /// the name of the registry
    pub name: String,
    /// whether it is the default registry
    pub default: bool,
    /// server address
    pub server: String,
    /// the optional username of the registry
    pub username: Option<String>,
    /// the optional password of the registry
    pub password: Option<String>,
    /// the optional token of the registry
    pub token: Option<String>,
}

/// `RegistryCredential` implements `Credential`
impl Credential for RegistryCredential {
    fn username(&self) -> Option<String> {
        self.username.to_owned()
    }

    fn password(&self) -> Option<String> {
        self.password.to_owned()
    }

    fn token(&self) -> Option<String> {
        self.token.to_owned()
    }
}

/// Access credentials for multiple code repositories.
/// indicating basic authentication when there is only a
/// username and password, otherwise bearer authentication.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RepositoryCredential {
    /// the name of the repository
    pub name: String,
    /// the driver for connecting to the repository
    pub driver: String,
    /// server address of the repository
    pub server: String,
    /// the optional username of the repository
    pub username: Option<String>,
    /// the optional password of the repository
    pub password: Option<String>,
    /// the optional token of the repository
    pub token: Option<String>,
}

/// `RepositoryCredential` implements `Credential`
impl Credential for RepositoryCredential {
    fn username(&self) -> Option<String> {
        self.username.to_owned()
    }

    fn password(&self) -> Option<String> {
        self.password.to_owned()
    }

    fn token(&self) -> Option<String> {
        self.token.to_owned()
    }
}

pub enum Scheme {
    /// Basic authentication is a simple authentication scheme built into the HTTP protocol.
    Basic,
    /// Bearer authentication (also called token authentication) is an HTTP authentication scheme
    /// that involves security tokens called bearer tokens. The name “Bearer authentication”
    /// can be understood as “give access to the bearer of this token.” The bearer token is a cryptic string,
    /// usually generated by the server in response to a login request.
    Bearer,
    /// Username or token is empty, or other cases
    Unknown,
}
