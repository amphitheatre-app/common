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

use crate::schema::{GitReference, RegisteredPartner};

use super::CharacterSpec;

/// the lead character in a story.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
pub struct Preface {
    /// The name of the character.
    pub name: String,
    /// The preface from registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<RegisteredPartner>,
    /// The preface from git repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<GitReference>,
    /// The preface from manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<CharacterSpec>,
}

impl Preface {
    pub fn registry(name: &str, registry: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            registry: Some(RegisteredPartner {
                registry: Some(registry.to_string()),
                version: version.to_string(),
            }),
            ..Preface::default()
        }
    }

    pub fn repository(repository: &str) -> Self {
        Self {
            name: repository.to_string(),
            repository: Some(GitReference::new(repository.to_string())),
            ..Preface::default()
        }
    }

    pub fn manifest(manifest: &CharacterSpec) -> Self {
        Self {
            name: manifest.meta.name.clone(),
            manifest: Some(manifest.clone()),
            ..Preface::default()
        }
    }
}
