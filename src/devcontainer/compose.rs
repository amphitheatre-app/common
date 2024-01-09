// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use super::types::{IntegerOrStringOrArray, StringOrArray};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonComposeBase {
    /// Application ports that are exposed by the container.
    /// This can be a single port or an array of ports. Each port can be a number or a string.
    /// A number is mapped to the same port on the host. A string is passed to Docker unchanged
    /// and can be used to map ports differently, e.g. "8000:8010".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_port: Option<IntegerOrStringOrArray>,
    /// The arguments required when starting in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_args: Option<Vec<String>>,
    /// Action to take when the user disconnects from the container in their editor.
    /// The default is to stop the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_action: Option<ShutdownAction>,
    /// Whether to overwrite the command specified in the image. The default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_command: Option<bool>,
    /// The path of the workspace folder inside the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folder: Option<String>,
    /// The --mount parameter for docker run. The default is to mount the project folder at /workspaces/$project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_mount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeContainer {
    /// The name of the docker-compose file(s) used to start the services.
    pub docker_compose_file: StringOrArray,
    /// The service you want to work on. T
    /// his is considered the primary container for your dev environment
    ///  which your editor will connect to.
    pub service: String,
    /// An array of services that should be started and stopped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_services: Option<Vec<String>>,
    /// The path of the workspace folder inside the container.
    /// This is typically the target path of a volume mount in the docker-compose.yml.
    pub workspace_folder: String,
    /// Action to take when the user disconnects from the primary container in their editor.
    ///  The default is to stop all of the compose containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_action: Option<ShutdownAction>,
    /// Whether to overwrite the command specified in the image. The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_command: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}
