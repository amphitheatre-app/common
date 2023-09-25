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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{self, GitReference, LocalPartner, RegistredPartner};

/// Specify Support for OpenAPI, it does not untagged.
/// Your character can depend on other characters from Registry or other registries,
/// git repositories, or subdirectories of project.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Partner {
    Registry(RegistredPartner),
    Repository(GitReference),
    Local(LocalPartner),
}

impl From<schema::Partner> for Partner {
    fn from(value: schema::Partner) -> Self {
        match value {
            schema::Partner::Registry(value) => Partner::Registry(value),
            schema::Partner::Repository(value) => Partner::Repository(value),
            schema::Partner::Local(value) => Partner::Local(value),
        }
    }
}
