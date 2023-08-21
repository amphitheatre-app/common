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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<DockerfileContainer>,

    // ## Docker Composer specific properties.
    //
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub compose: Option<ComposeContainer>,
}

#[cfg(test)]
mod test {
    use crate::devcontainer;

    #[test]
    fn test_deserialize_dev_container() {
        let data = r#"{
                "build": {
                    "dockerfile": "./Dockerfile",
                    "context": "."
                },
                "features": {
                    "ghcr.io/devcontainers/features/common-utils:2": {
                        "installZsh": "true",
                        "username": "vscode",
                        "userUid": "1000",
                        "userGid": "1000",
                        "upgradePackages": "true"
                    },
                    "ghcr.io/devcontainers/features/rust:1": "latest",
                    "ghcr.io/devcontainers/features/git:1": {
                        "version": "latest",
                        "ppa": "false"
                    }
                },

                // Use 'forwardPorts' to make a list of ports inside the container available locally.
                // "forwardPorts": [],

                // Use 'postCreateCommand' to run commands after the container is created.
                // "postCreateCommand": "rustc --version",

                // Set `remoteUser` to `root` to connect as root instead.
                "remoteUser": "vscode"
            }"#;

        let container: super::DevContainer = devcontainer::from_str(data).unwrap();

        assert_eq!(container.build.is_some(), true);
        assert_eq!(
            container.build.as_ref().unwrap().dockerfile,
            "./Dockerfile".to_string()
        );
        assert_eq!(container.common.features.is_some(), true);
        assert_eq!(container.common.remote_user, Some("vscode".into()));
    }
}
