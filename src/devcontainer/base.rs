// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

use super::{
    common::DevContainerCommon,
    compose::{ComposeContainer, NonComposeBase},
    container::{DockerfileContainer, ImageContainer},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DevContainer {
    // ## General properties, common to all.
    #[serde(flatten)]
    pub common: DevContainerCommon,

    // ## Image or Dockerfile specific properties.
    //
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub non_compose_base: Option<NonComposeBase>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub image_container: Option<ImageContainer>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub dockerfile_container: Option<DockerfileContainer>,

    // ## Docker Composer specific properties.
    //
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub compose: Option<ComposeContainer>,
}
