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

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::{self, Build, Deploy, Metadata};

use super::Partner;

#[derive(
    Clone, CustomResource, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq, Validate,
)]
#[kube(group = "amphitheatre.app", version = "v1", kind = "Character")]
pub struct CharacterSpec {
    /// Contains all the information about a character.
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
    /// Whether this character is live or not
    pub live: bool,
    /// Exit after one sync with live mode
    pub once: bool,
}

impl From<&schema::Character> for CharacterSpec {
    fn from(value: &schema::Character) -> CharacterSpec {
        // Convert resource::Partner to schema::Partner
        let partners = value
            .partners
            .clone()
            .map(|partners| {
                partners
                    .into_iter()
                    .map(|(name, partner)| (name, Partner::from(partner)))
                    .collect()
            })
            .unwrap_or_default();

        CharacterSpec {
            meta: value.meta.clone(),
            build: value.build.clone(),
            deploy: value.deploy.clone(),
            partners: Some(partners),
            live: false,
            once: false,
        }
    }
}
