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

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use super::CredentialConfiguration;

/// The `Cluster` is used to store the server address and access token of the cluster.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Cluster {
    /// the name of the cluster
    pub title: String,
    /// the server address of the cluster
    pub server: String,
    /// the optional access token of the cluster
    pub token: Option<String>,
    /// the optional credentials used in the cluster
    pub credentials: Option<CredentialConfiguration>,
}

/// `ContextConfiguration` is used to store the configuration of the context.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContextConfiguration {
    /// the selected context name of the cluster
    current: Option<String>,
    /// the list of connectable clusters
    clusters: HashMap<String, Cluster>,
}

impl ContextConfiguration {
    /// Get the current context.
    pub fn current(&self) -> Option<&Cluster> {
        if let Some(name) = &self.current {
            return self.clusters.get(name).to_owned();
        }

        None
    }

    // Remove a context from the list of contexts by name and save the configuration.
    pub fn delete(&mut self, name: &str) -> Result<()> {
        if !self.clusters.contains_key(name) {
            return Err(anyhow!("Context with name `{}` does not exist", name));
        }

        self.clusters.remove(name);
        println!("Deleted context with name `{}`", name);

        Ok(())
    }

    /// impl iter method for ContextConfiguration
    pub fn iter(&self) -> impl Iterator<Item = &Cluster> {
        self.clusters.values()
    }
}
