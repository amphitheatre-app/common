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

use k8s_openapi::api::core::v1::{ContainerPort, ServicePort};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Defines the behavior of a service
#[derive(Clone, Debug, Deserialize, Eq, Serialize, JsonSchema, PartialEq)]
pub struct Service {
    /// Type determines how the Service is exposed. Defaults to ClusterIP.
    /// Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The list of ports that are exposed by this service.
    pub ports: Vec<Port>,
}

pub enum Error {}

impl TryInto<Vec<ContainerPort>> for Service {
    type Error = Error;

    fn try_into(self) -> Result<Vec<ContainerPort>, Self::Error> {
        todo!()
    }
}

/// List of ports to expose from the container.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, JsonSchema, PartialEq)]
pub struct Port {
    /// The port that will be exposed by this service.
    pub port: i32,
    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Exposes HTTP and HTTPS routes from outside the cluster to services within the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose: Option<bool>,
}

// impl Into<ContainerPort> for &Port {
//     fn into(self) -> ContainerPort {
//         ContainerPort {
//             container_port: self.port,
//             protocol: self.protocol.clone(),
//             ..Default::default()
//         }
//     }
// }

impl From<&Port> for ContainerPort {
    fn from(val: &Port) -> Self {
        ContainerPort {
            container_port: val.port,
            protocol: val.protocol.clone(),
            ..Default::default()
        }
    }
}

impl From<&Port> for ServicePort {
    fn from(val: &Port) -> Self {
        ServicePort {
            port: val.port,
            protocol: val.protocol.clone(),
            ..Default::default()
        }
    }
}

// impl Into<ServicePort> for &Port {
//     fn into(self) -> ServicePort {
//         ServicePort {
//             port: self.port,
//             protocol: self.protocol.clone(),
//             ..Default::default()
//         }
//     }
// }
