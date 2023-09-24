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
use std::default;
use std::hash::Hash;

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::build::Build;
use super::service::Service;
use super::GitReference;

/// Contains all the information about a character, as loaded from a `.amp.toml`.
#[derive(
    Clone, CustomResource, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq, Validate,
)]
#[kube(group = "amphitheatre.app", version = "v1", kind = "Character")]
pub struct Manifest {
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

    /// Specify the external image to be launched when it’s a reference kind of manifest.
    /// Or by default, leave it empty and Amphitheatre will automatically create it
    /// based on the current registry and name.
    ///
    /// The image must follow the Open Container Specification addressable image format.
    /// such as: [<registry>/][<project>/]<image>[:<tag>|@<digest>].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// overrides the default command declared by the container image
    /// (i.e. by Dockerfile’s CMD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Defines environment variables set in the container. Any boolean values:
    /// true, false, yes, no, SHOULD be enclosed in quotes to ensure they are
    /// not converted to True or False by the YAML parser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// Depend on other partners from other repositories,
    /// or subdirectories on your local file system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<HashMap<String, Partner>>,

    /// Defines the behavior of a service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,

    /// Describes how images are built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum EitherCharacter {
    Git(GitReference),
    Manifest(String),
    Name(String),
}

pub type Preface = EitherCharacter;
pub type Partner = EitherCharacter;

impl default::Default for EitherCharacter {
    fn default() -> Self {
        Self::Manifest("".to_string())
    }
}
