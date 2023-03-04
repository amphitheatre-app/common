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

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Character {
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
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.repository)
    }
}
