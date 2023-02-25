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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Character {
    /// The name of the character.
    pub name: String,
    /// The version of the character.
    pub version: Option<String>,
    /// The authors of the character.
    pub authors: Option<Vec<String>>,
    /// A description of the character.
    pub description: Option<String>,
    /// Path to the character's README file.
    pub readme: Option<String>,
    /// URL of the character homepage.
    pub homepage: Option<String>,
    /// URL of the character source repository.
    /// e.g. https://github.com/amphitheatre-app/amphitheatre.git.
    pub repository: String,
    /// The character license.
    pub license: Option<String>,
    /// Path to the text of the license.
    pub license_file: Option<String>,
    /// Keywords for the character.
    pub keywords: Option<Vec<String>>,
    /// Categories of the character.
    pub categories: Option<Vec<String>>,
    /// Files to exclude when publishing.
    pub exclude: Option<Vec<String>>,
    /// Files to include when publishing.
    pub include: Option<Vec<String>>,
    /// Can be used to prevent publishing the character.
    pub publish: Option<Vec<String>>,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.repository)
    }
}
