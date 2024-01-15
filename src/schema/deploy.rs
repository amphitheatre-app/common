// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use crate::utils::kubernetes::to_env_var;
use k8s_openapi::api::core::v1::{ContainerPort, EnvVar, ServicePort};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::Service;

/// Describes how images are deploy.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, Serialize, PartialEq, ToSchema)]
pub struct Deploy {
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
    /// Defines environment variables set in the container. Any boolean values:
    /// true, false, yes, no, SHOULD be enclosed in quotes to ensure they are
    /// not converted to True or False by the YAML parser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    /// Arguments passed to the container command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Defines the behavior of a service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

/// Helpers for Kubernetes resources.
impl Deploy {
    pub fn env(&self) -> Option<Vec<EnvVar>> {
        return self.env.as_ref().map(to_env_var);
    }

    pub fn container_ports(&self) -> Option<Vec<ContainerPort>> {
        let services = self.services.as_ref()?;
        let mut ports: Vec<ContainerPort> = vec![];

        for service in services {
            let mut items = service.ports.iter().map(|p| p.into()).collect();
            ports.append(&mut items);
        }

        Some(ports).filter(|v| v.is_empty())
    }

    pub fn service_ports(&self) -> Option<Vec<ServicePort>> {
        let services = self.services.as_ref()?;
        let mut ports: Vec<ServicePort> = vec![];

        for service in services {
            let mut items = service
                .ports
                .iter()
                .filter(|p| p.expose.unwrap_or_default())
                .map(|p| p.into())
                .collect();
            ports.append(&mut items);
        }

        Some(ports).filter(|v| v.is_empty())
    }
}
