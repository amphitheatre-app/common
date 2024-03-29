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

use super::GitReference;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Your character can depend on other characters from Registry or other registries,
/// git repositories, or subdirectories of local filesystem.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema, ToSchema)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Partner {
    Registry(RegisteredPartner),
    Repository(GitReference),
    Local(LocalPartner),
}

/// A partner that pulls a character from a registry.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, JsonSchema, ToSchema)]
pub struct RegisteredPartner {
    /// The registry to pull the character from. Defaults to Catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    /// The version of the character to pull.
    pub version: String,
}

/// A partner that pulls a character from a local path.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, JsonSchema, ToSchema)]
pub struct LocalPartner {
    /// The path to the character.
    pub path: String,
}
