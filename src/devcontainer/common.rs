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

// DevContainer Definition schema
// see: https://github.com/devcontainers/spec/blob/main/schemas/devContainer.base.schema.json

use std::collections::HashMap;

use serde_json::Value;

use super::{
    lifecycle::LifecycleScripts, mount::StringOrMount, ports_attributes::PortAttributes,
    requirements::HostRequirements, types::IntegerOrString,
};

pub struct DevContainerCommon {
    /// The name for the dev container which can be displayed to the user.
    pub name: Option<String>,
    /// Features to add to the dev container.
    pub features: Option<HashMap<String, String>>,
    /// ray consisting of the Feature id (without the semantic version) of Features
    /// in the order the user wants them to be installed.
    pub override_feature_install_order: Option<Vec<String>>,
    /// Ports that are forwarded from the container to the local machine.
    /// Can be an integer port number, or a string of the format \"host:port_number\"."
    pub forward_ports: Option<Vec<IntegerOrString>>,
    /// Object that maps a port number, "host:port" value, range, or regular expression to a set of default options.
    pub ports_attributes: Option<HashMap<String, PortAttributes>>,
    /// Default options for ports, port ranges, and hosts that aren’t configured using `PortsAttributes`.
    pub other_ports_attributes: Option<PortAttributes>,
    /// Controls whether on Linux the container's user should be updated with the local user's UID and GID.
    /// On by default when opening from a local folder.
    pub update_remote_user_uid: Option<bool>,
    /// Container environment variables.
    pub container_env: Option<HashMap<String, String>>,
    /// The user the container will be started with. The default is the user on the Docker image.
    pub container_user: Option<String>,
    /// ount points to set up when creating the container.
    ///  See Docker's documentation for the --mount option for the supported syntax.
    pub mounts: Option<Vec<StringOrMount>>,
    /// Passes the --init flag when creating the dev container.
    pub init: Option<bool>,
    /// Passes the --privileged flag when creating the dev container.
    pub privileged: Option<bool>,
    /// Passes docker capabilities to include when creating the dev container.
    pub cap_add: Option<Vec<String>>,
    /// Passes docker security options to include when creating the dev container.
    pub security_opt: Option<Vec<String>>,
    /// Remote environment variables to set for processes spawned in the container
    /// including lifecycle scripts and any remote editor/IDE server process.
    pub remote_env: Option<HashMap<String, String>>,
    /// The username to use for spawning processes in the container
    /// including lifecycle scripts and any remote editor/IDE server process.
    /// The default is the same user as the container.
    pub remote_user: Option<String>,
    /// When creating or working with a dev container, you may need different
    /// commands to be run at different points in the container’s lifecycle.
    pub lifecycle_scripts: Option<LifecycleScripts>,
    /// User environment probe to run. The default is "loginInteractiveShell"
    pub user_env_probe: Option<UserEnvProbe>,
    /// Host hardware requirements.
    pub host_requirements: Option<HostRequirements>,
    pub customizations: Option<HashMap<String, Value>>,
    pub additional_properties: Option<HashMap<String, Value>>,
}

pub enum UserEnvProbe {
    None,
    LoginShell,
    // TODO: default
    LoginInteractiveShell,
    InteractiveShell,
}
