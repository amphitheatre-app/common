// Copyright 2023 The Amphitheatre Authors.
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

use std::collections::HashMap;

use k8s_openapi::api::core::v1::EnvVar;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::utils::kubernetes::to_env_var;

/// Describes how images are built.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq)]
pub struct Build {
    /// Which method to use to build the image.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub method: Option<BuildMethod>,
    /// Directory containing the artifact's sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Environment variables, in the key=value form, passed to the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    /// Arguments passed to the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Files to exclude when building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// Files to include when building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

/// Helpers for Kubernetes resources.
impl Build {
    pub fn env(&self) -> Option<Vec<EnvVar>> {
        return self.env.as_ref().map(to_env_var);
    }
}

/// Which method to use to build the image.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
pub enum BuildMethod {
    Dockerfile(DockerfileConfig),
    Buildpacks(BuildpacksConfig),
}

impl Default for BuildMethod {
    fn default() -> Self {
        Self::Buildpacks(BuildpacksConfig::default())
    }
}

/// The configuration for building an image using a Dockerfile.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
pub struct DockerfileConfig {
    /// Locates the Dockerfile relative to workspace.
    pub dockerfile: String,
}

/// The configuration for building an image using Cloud Native Buildpacks.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
pub struct BuildpacksConfig {
    /// Builds images using Cloud Native Buildpacks
    /// Builder image (default "cnbs/sample-builder:jammy")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    /// Buildpacks to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildpacks: Option<Vec<String>>,
}

const DEFAULT_BP_BUILDER: &str = "gcr.io/buildpacks/builder:v1";

impl Default for BuildpacksConfig {
    fn default() -> Self {
        Self {
            builder: Some(String::from(DEFAULT_BP_BUILDER)),
            buildpacks: None,
        }
    }
}

impl Build {
    pub fn builder(&self) -> String {
        let builder = match &self.method {
            Some(BuildMethod::Dockerfile(_)) => None,
            Some(BuildMethod::Buildpacks(config)) => config.builder.clone(),
            None => None,
        };

        builder.unwrap_or_else(|| String::from(DEFAULT_BP_BUILDER))
    }
}
