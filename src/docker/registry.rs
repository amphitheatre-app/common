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

use std::str::FromStr;

use oci_distribution::client::ClientConfig;
use oci_distribution::secrets::RegistryAuth;
use oci_distribution::{Client, Reference};
use tracing::{debug, error, warn};

use super::DockerCredential;

/// Check if the docker image exists on remote registry.
pub async fn exists(image: &str, credential: Option<DockerCredential>) -> anyhow::Result<bool> {
    let mut client = Client::new(ClientConfig::default());
    let reference = Reference::from_str(image)?;

    let auth = match credential {
        Some(DockerCredential::UsernamePassword(username, password)) => {
            debug!("Found docker credentials");
            RegistryAuth::Basic(username, password)
        }
        Some(DockerCredential::IdentityToken(_)) => {
            warn!("Cannot use contents of docker config, identity token not supported. Using anonymous auth");
            RegistryAuth::Anonymous
        }
        None => RegistryAuth::Anonymous,
    };

    match client.fetch_manifest_digest(&reference, &auth).await {
        Ok(digest) => {
            debug!("Fetched manifest digest {} for image {}", digest, image);
            Ok(true)
        }
        Err(err) => {
            error!("OciDistributionError: {}", err.to_string());
            Ok(false)
        }
    }
}
