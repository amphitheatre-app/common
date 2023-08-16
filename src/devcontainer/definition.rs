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

pub enum IntegerOrString {
    /// minimum: 0, maximum: 65535
    Integer(i32),
    /// pattern: ^([a-z0-9-]+):(\\d{1,5})$
    String(String),
}

/// The PortAttributes properties allow you to map default port options
/// for one or more manually or automatically forwarded ports.
pub struct PortAttributes {
    /// Defines the action that occurs when the port is discovered for automatic forwarding
    pub on_auto_forward: Option<OnAutoForward>,
    /// Automatically prompt for elevation (if needed) when this port is forwarded.
    /// Elevate is required if the local port is a privileged port.
    pub elevate_if_needed: Option<bool>,
    /// Label that will be shown in the UI for this port.
    // TODO: default: Application
    pub label: Option<String>,
    /// When true, a modal dialog will show if the chosen local port isn't used for forwarding.
    pub require_local_port: Option<bool>,
    /// The protocol to use when forwarding this port.
    pub protocol: Option<Protocol>,
}

impl Default for PortAttributes {
    fn default() -> Self {
        Self {
            on_auto_forward: Some(OnAutoForward::Notify),
            elevate_if_needed: Some(false),
            label: Some("Application".to_string()),
            require_local_port: Some(false),
            protocol: None,
        }
    }
}

/// Defines the action that occurs when the port is discovered for automatic forwarding
pub enum OnAutoForward {
    /// Shows a notification when a port is automatically forwarded.
    // TODO: default
    Notify,
    /// Opens the browser when the port is automatically forwarded.
    /// Depending on your settings, this could open an embedded browser.
    OpenBrowser,
    /// Opens the browser when the port is automatically forwarded,
    /// but only the first time the port is forward during a session.
    /// Depending on your settings, this could open an embedded browser.
    OpenBrowserOnce,
    /// Opens a preview in the same window when the port is automatically forwarded.
    OpenPreview,
    /// Shows no notification and takes no action when this port is automatically forwarded.
    Silent,
    /// This port will not be automatically forwarded.
    Ignore,
}

/// The Protocol to use when forwarding a port.
pub enum Protocol {
    Http,
    Https,
}

/// A mount can be a bind mount or a volume mount.
pub enum StringOrMount {
    String(String),
    Mount(Mount),
}

pub struct Mount {
    /// Mount type.
    pub kind: MountKind,
    /// Mount source.
    pub source: String,
    /// Mount target.
    pub target: String,
}

/// Mount type.
pub enum MountKind {
    Bind,
    Volume,
}

/// When creating or working with a dev container, you may need different
/// commands to be run at different points in the container’s lifecycle.
pub struct LifecycleScripts {
    /// A command to run locally (i.e Your host machine, cloud VM) before anything else.
    /// This command is run before "onCreateCommand"". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub initialize_command: Option<LifecycleScript>,
    /// A command to run when creating the container.
    /// This command is run after "initializeCommand" and before "updateContentCommand".
    /// If this is a single string, it will be run in a shell. If this is an array of strings,
    /// it will be run as a single command without shell. If this is an object,
    /// each provided command will be run in parallel.
    pub on_create_command: Option<LifecycleScript>,
    /// A command to run when creating the container and rerun when the workspace content
    /// was updated while creating the container. This command is run after "onCreateCommand"
    /// and before "postCreateCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub update_content_command: Option<LifecycleScript>,
    /// A command to run after creating the container. This command is run after "updateContentCommand"
    /// and before "postStartCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub post_create_command: Option<LifecycleScript>,
    /// A command to run after starting the container. This command is run after "postCreateCommand"
    /// and before "postAttachCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.",
    pub post_start_command: Option<LifecycleScript>,
    /// A command to run when attaching to the container. This command is run after "postStartCommand".
    /// If this is a single string, it will be run in a shell. If this is an array of strings,
    /// it will be run as a single command without shell. If this is an object,
    /// each provided command will be run in parallel.
    pub post_attach_command: Option<LifecycleScript>,
    /// The user command to wait for before continuing execution in the background
    /// while the UI is starting up. The default is "updateContentCommand"
    pub wait_for: Option<WaitFor>,
}

pub enum LifecycleScript {
    String(String),
    Array(Vec<String>),
    Object(HashMap<String, String>),
}

pub enum WaitFor {
    InitializeCommand,
    OnCreateCommand,
    UpdateContentCommand,
    PostCreateCommand,
    PostStartCommand,
}

pub enum UserEnvProbe {
    None,
    LoginShell,
    // TODO: default
    LoginInteractiveShell,
    InteractiveShell,
}

/// Host hardware requirements.
pub struct HostRequirements {
    /// Number of required CPUs. minimum: 1
    pub cpus: Option<u32>,
    /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    pub memory: Option<String>,
    /// Amount of required disk space in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    pub storage: Option<String>,
    /// Indicates whether a GPU is required.
    /// The string "optional" indicates that a GPU is optional.
    /// An object value can be used to configure more detailed requirements.
    pub gpu: Option<GPUVar>,
}

pub enum GPUVar {
    /// Indicates whether a GPU is required.
    Boolean(bool),
    /// The string "optional" indicates that a GPU is optional.
    String(String),
    /// An object value can be used to configure more detailed requirements.
    Config(GPUConfig),
}

pub struct GPUConfig {
    /// Number of required cores. minimum: 1
    pub cores: Option<u32>,
    /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    pub memory: Option<String>,
}

pub struct NonComposeBase {
    /// Application ports that are exposed by the container.
    /// This can be a single port or an array of ports. Each port can be a number or a string.
    /// A number is mapped to the same port on the host. A string is passed to Docker unchanged
    /// and can be used to map ports differently, e.g. "8000:8010".
    pub app_port: Option<IntegerOrStringOrArray>,
    /// The arguments required when starting in the container.
    pub run_args: Option<Vec<String>>,
    /// Action to take when the user disconnects from the container in their editor.
    /// The default is to stop the container.
    pub shutdown_action: Option<ShutdownAction>,
    /// Whether to overwrite the command specified in the image. The default is true.
    pub override_command: Option<bool>,
    /// The path of the workspace folder inside the container.
    pub workspace_folder: Option<String>,
    /// The --mount parameter for docker run. The default is to mount the project folder at /workspaces/$project.
    pub workspace_mount: Option<String>,
}

pub enum IntegerOrStringOrArray {
    Integer(u32),
    String(String),
    Array(Vec<IntegerOrString>),
}

pub enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}

pub struct DockerfileContainer {
    /// The location of the Dockerfile that defines the contents of the container.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub dockerfile: String,
    /// The location of the context folder for building the Docker image.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub context: Option<String>,
    /// Docker build-related options.
    pub options: Option<BuildOptions>,
}

pub struct BuildOptions {
    /// Target stage in a multi-stage build.
    pub target: Option<String>,
    /// Build arguments.
    pub args: Option<HashMap<String, String>>,
    /// The image to consider as a cache. Use an array to specify multiple images.
    pub cache_from: Option<StringOrArray>,
}

pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

pub struct ImageContainer {
    /// The docker image that will be used to create the container.
    pub image: String,
}

pub struct ComposeContainer {
    /// The name of the docker-compose file(s) used to start the services.
    pub docker_compose_file: StringOrArray,
    /// The service you want to work on. T
    /// his is considered the primary container for your dev environment
    ///  which your editor will connect to.
    pub service: String,
    /// An array of services that should be started and stopped.
    pub run_services: Option<Vec<String>>,
    /// The path of the workspace folder inside the container.
    /// This is typically the target path of a volume mount in the docker-compose.yml.
    pub workspace_folder: String,
    /// Action to take when the user disconnects from the primary container in their editor.
    ///  The default is to stop all of the compose containers.
    pub shutdown_action: Option<ShutdownAction>,
    /// Whether to overwrite the command specified in the image. The default is false.
    pub override_command: Option<bool>,
}
