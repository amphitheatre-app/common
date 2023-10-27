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

use std::{collections::HashMap, fs::read_to_string, path::Path};

use anyhow::anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{Build, Deploy, Partner};

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq, Validate)]
pub struct Character {
    /// Contains all the information about a character.
    #[serde(flatten)]
    pub meta: Metadata,
    /// Describes how images are built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    /// Describes how images are deploy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Deploy>,
    /// Depend on other partners from other repositories,
    /// or subdirectories on your local file system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<HashMap<String, Partner>>,
}

impl Character {
    pub fn new(name: &str) -> Self {
        Self {
            meta: Metadata {
                name: name.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Load the character from the specified file.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let content = read_to_string(path).map_err(|e| anyhow!(e.to_string()))?;
        toml::from_str(&content).map_err(|e| anyhow!(e.to_string()))
    }
}

/// Contains all the information about a character.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq, Validate)]
pub struct Metadata {
    /// The name of the character.
    pub name: String,
    /// The version of the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The authors of the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    /// A description of the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the character documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    /// Path to the character's README file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    /// URL of the character homepage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// URL of the character source repository.
    /// e.g. https://github.com/amphitheatre-app/amphitheatre.git.
    pub repository: String,
    /// The character license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Path to the text of the license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_file: Option<String>,
    /// Keywords for the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Categories of the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Can be used to prevent publishing the character.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<Vec<String>>,
}
