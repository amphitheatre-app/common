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

use serde::{Deserialize, Serialize};

use super::build::Build;
use super::character::Character;
use super::partner::Partner;
use super::service::Service;

/// The .amp.toml file for each character is called its manifest. It is written
/// in the TOML format. It contains metadata that is needed to compile the character.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    /// Defines a character.
    pub character: Character,

    /// Defines environment variables set in the container. Any boolean values:
    /// true, false, yes, no, SHOULD be enclosed in quotes to ensure they are
    /// not converted to True or False by the YAML parser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<HashMap<String, String>>,

    /// Depend on other partners from other repositories, or subdirectories on
    /// your local file system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<Vec<HashMap<String, Partner>>>,

    /// Defines the behavior of a service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,

    /// Describes how images are built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}
