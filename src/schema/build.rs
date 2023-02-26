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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Describes how images are built.
#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize, PartialEq)]
pub struct Build {
    /// Directory containing the artifact's sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Environment variables, in the key=value form, passed to the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// Builds images using kaniko.
    /// Locates the Dockerfile relative to workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,

    /// Builds images using Cloud Native Buildpacks,
    /// which Cluster Builder image used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,

    /// Files to exclude when building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,

    /// Files to include when building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}
