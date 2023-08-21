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

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    lifecycle::LifecycleScripts, mount::StringOrMount, ports_attributes::PortAttributes,
    requirements::HostRequirements, types::IntegerOrString,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevContainerCommon {
    /// The name for the dev container which can be displayed to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Features to add to the dev container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<HashMap<String, Value>>,
    /// ray consisting of the Feature id (without the semantic version) of Features
    /// in the order the user wants them to be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_feature_install_order: Option<Vec<String>>,
    /// Ports that are forwarded from the container to the local machine.
    /// Can be an integer port number, or a string of the format \"host:port_number\"."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_ports: Option<Vec<IntegerOrString>>,
    /// Object that maps a port number, "host:port" value, range, or regular expression to a set of default options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports_attributes: Option<HashMap<String, PortAttributes>>,
    /// Default options for ports, port ranges, and hosts that aren’t configured using `PortsAttributes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_ports_attributes: Option<PortAttributes>,
    /// Controls whether on Linux the container's user should be updated with the local user's UID and GID.
    /// On by default when opening from a local folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_remote_user_uid: Option<bool>,
    /// Container environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_env: Option<HashMap<String, String>>,
    /// The user the container will be started with. The default is the user on the Docker image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_user: Option<String>,
    /// ount points to set up when creating the container.
    ///  See Docker's documentation for the --mount option for the supported syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<StringOrMount>>,
    /// Passes the --init flag when creating the dev container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// Passes the --privileged flag when creating the dev container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Passes docker capabilities to include when creating the dev container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// Passes docker security options to include when creating the dev container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// Remote environment variables to set for processes spawned in the container
    /// including lifecycle scripts and any remote editor/IDE server process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_env: Option<HashMap<String, String>>,
    /// The username to use for spawning processes in the container
    /// including lifecycle scripts and any remote editor/IDE server process.
    /// The default is the same user as the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_user: Option<String>,
    /// When creating or working with a dev container, you may need different
    /// commands to be run at different points in the container’s lifecycle.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub lifecycle_scripts: Option<LifecycleScripts>,
    /// User environment probe to run. The default is "loginInteractiveShell"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_env_probe: Option<UserEnvProbe>,
    /// Host hardware requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_requirements: Option<HostRequirements>,
    /// Tool-specific configuration. Each tool should use a JSON object subproperty
    ///  with a unique name to group its customizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserEnvProbe {
    None,
    LoginShell,
    // TODO: default
    LoginInteractiveShell,
    InteractiveShell,
}

#[cfg(test)]
mod test {
    use crate::devcontainer::{lifecycle::LifecycleScript, ports_attributes::OnAutoForward};

    #[test]
    fn test_dev_container_common() {
        let data = r#"{
            "name": "devcontainer",
            "features": {
                "vscode": {
                    "extensions": [
                        "ms-python.python",
                        "ms-toolsai.jupyter"
                    ]
                }
            },
            "overrideFeatureInstallOrder": [
                "vscode"
            ],
            "forwardPorts": [
                8888
            ],
            "portsAttributes": {
                "8888": {
                    "label": "8888",
                    "protocol": "http",
                    "onAutoForward": "notify"
                }
            },
            "otherPortsAttributes": {
                "onAutoForward": "notify"
            },
            "updateRemoteUserUid": true,
            "containerEnv": {
                "PYTHONPATH": "/workspace"
            },
            "containerUser": "vscode",
            "mounts": [{
                "source": "/var/run/docker.sock",
                "target": "/var/run/docker.sock",
                "type": "Volume"
            }],
            "init": true,
            "privileged": true,
            "capAdd": [
                "SYS_PTRACE"
            ],
            "securityOpt": [
                "seccomp=unconfined"
            ],
            "remoteEnv": {
                "PYTHONPATH": "/workspace"
            },
            "remoteUser": "vscode",
            "postCreateCommand": "pip install -r requirements.txt",
            "postStartCommand": "jupyter notebook --ip"
        }"#;

        let common: super::DevContainerCommon = serde_json::from_str(data).unwrap();

        assert_eq!(common.name, Some("devcontainer".into()));
        assert_eq!(common.features.is_some(), true);
        let features = common.features.unwrap();
        assert_eq!(
            features.get("vscode"),
            Some(&serde_json::json!({
                "extensions": [
                    "ms-python.python",
                    "ms-toolsai.jupyter"
                ]
            }))
        );
        assert_eq!(common.override_feature_install_order.unwrap().len(), 1);
        assert_eq!(common.forward_ports.unwrap().len(), 1);
        assert_eq!(common.ports_attributes.unwrap().len(), 1);
        assert_eq!(
            common.other_ports_attributes.unwrap().on_auto_forward.unwrap(),
            OnAutoForward::Notify
        );
        assert_eq!(common.update_remote_user_uid.unwrap(), true);
        assert_eq!(common.container_env.unwrap().len(), 1);
        assert_eq!(common.container_user.unwrap(), "vscode");
        assert_eq!(common.mounts.unwrap().len(), 1);
        assert_eq!(common.init.unwrap(), true);
        assert_eq!(common.privileged.unwrap(), true);
        assert_eq!(common.cap_add.unwrap().len(), 1);
        assert_eq!(common.security_opt.unwrap().len(), 1);
        assert_eq!(common.remote_env.unwrap().len(), 1);
        assert_eq!(common.remote_user.unwrap(), "vscode");

        assert_eq!(common.lifecycle_scripts.is_some(), true);
        let scripts = common.lifecycle_scripts.as_ref().unwrap();

        assert_eq!(
            scripts.post_create_command,
            Some(LifecycleScript::String("pip install -r requirements.txt".into()))
        );
        assert_eq!(
            scripts.post_start_command,
            Some(LifecycleScript::String("jupyter notebook --ip".into()))
        );
    }
}
