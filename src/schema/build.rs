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

use std::collections::HashMap;

use crate::utils::kubernetes::to_env_var;
use k8s_openapi::api::core::v1::EnvVar;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Describes how images are built.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq, ToSchema)]
pub struct Build {
    /// The configuration for building an image using a Dockerfile.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<DockerfileConfig>,
    /// The configuration for building an image using Cloud Native Buildpacks.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub buildpacks: Option<BuildpacksConfig>,
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
    /// The platforms to build the image for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
}

impl Build {
    pub fn env(&self) -> Option<Vec<EnvVar>> {
        return self.env.as_ref().map(to_env_var);
    }

    pub fn method(&self) -> BuildMethod {
        if self.dockerfile.is_some() {
            return BuildMethod::Dockerfile;
        }

        BuildMethod::Buildpacks
    }
}

/// Which method to use to build the image.
pub enum BuildMethod {
    Dockerfile,
    Buildpacks,
}

/// The configuration for building an image using a Dockerfile.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema, ToSchema)]
pub struct DockerfileConfig {
    /// Locates the Dockerfile relative to workspace.
    pub dockerfile: String,
}

impl Default for DockerfileConfig {
    fn default() -> Self {
        Self {
            dockerfile: "Dockerfile".to_string(),
        }
    }
}

/// The configuration for building an image using Cloud Native Buildpacks.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema, ToSchema)]
pub struct BuildpacksConfig {
    /// Builds images using Cloud Native Buildpacks,
    /// default builder is `gcr.io/buildpacks/builder:v1`
    pub builder: String,
    /// Buildpacks to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildpacks: Option<Vec<String>>,
}

const DEFAULT_BP_BUILDER: &str = "gcr.io/buildpacks/builder:v1";

impl Default for BuildpacksConfig {
    fn default() -> Self {
        Self {
            builder: String::from(DEFAULT_BP_BUILDER),
            buildpacks: None,
        }
    }
}
