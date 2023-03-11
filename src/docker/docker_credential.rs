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

// Heavily inspired by [keirlawson/docker_credential](https://github.com/keirlawson/docker_credential)

use std::str::FromStr;

use data_encoding::BASE64;
use oci_distribution::Reference;

use super::errors::CredentialError;
use super::DockerConfig;

type Result<T> = std::result::Result<T, CredentialError>;

/// A docker credential, either a single identity token or a username/password pair.
#[derive(Debug, PartialEq)]
pub enum DockerCredential {
    IdentityToken(String),
    UsernamePassword(String, String),
}

/// Retrieve a user's docker credential via DockerConfig (config.json).
pub fn get_credential(config: &DockerConfig, image: &str) -> Result<DockerCredential> {
    let reference = Reference::from_str(image).map_err(CredentialError::ReferenceParseError)?;
    let image_registry = reference
        .resolve_registry()
        .strip_suffix('/')
        .unwrap_or_else(|| reference.resolve_registry());

    if let Some(auth) = config.get_auth(image_registry) {
        return decode_auth(auth);
    }

    Err(CredentialError::NoCredentialConfigured)
}

fn decode_auth(encoded_auth: &str) -> Result<DockerCredential> {
    let decoded = BASE64
        .decode(encoded_auth.as_bytes())
        .map_err(|_| CredentialError::CredentialDecodingError)?;
    let decoded = std::str::from_utf8(&decoded).map_err(|_| CredentialError::CredentialDecodingError)?;

    let parts: Vec<&str> = decoded.splitn(2, ':').collect();
    let username = String::from(*parts.first().unwrap());
    let password = String::from(*parts.get(1).ok_or(CredentialError::CredentialDecodingError)?);

    Ok(DockerCredential::UsernamePassword(username, password))
}
