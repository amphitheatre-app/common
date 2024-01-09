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

/// Host hardware requirements.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HostRequirements {
    /// Number of required CPUs. minimum: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<u32>,
    /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// Amount of required disk space in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Indicates whether a GPU is required.
    /// The string "optional" indicates that a GPU is optional.
    /// An object value can be used to configure more detailed requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu: Option<GPUVar>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GPUVar {
    /// Indicates whether a GPU is required.
    Boolean(bool),
    /// The string "optional" indicates that a GPU is optional.
    String(String),
    /// An object value can be used to configure more detailed requirements.
    Config(GPUConfig),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GPUConfig {
    /// Number of required cores. minimum: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,
    /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_host_requirements() {
        use serde_json::json;

        let json = json!({
            "cpus": 1,
            "memory": "1gb",
            "storage": "1gb",
            "gpu": {
                "cores": 1,
                "memory": "1gb"
            }
        });
        let host_requirements: HostRequirements = serde_json::from_value(json).unwrap();
        assert_eq!(
            host_requirements,
            HostRequirements {
                cpus: Some(1),
                memory: Some("1gb".to_string()),
                storage: Some("1gb".to_string()),
                gpu: Some(GPUVar::Config(GPUConfig {
                    cores: Some(1),
                    memory: Some("1gb".to_string()),
                })),
            }
        );
    }

    #[test]
    fn test_host_requirements_gpu_boolean() {
        use serde_json::json;

        let json = json!({
            "cpus": 1,
            "memory": "1gb",
            "storage": "1gb",
            "gpu": true
        });
        let host_requirements: HostRequirements = serde_json::from_value(json).unwrap();
        assert_eq!(
            host_requirements,
            HostRequirements {
                cpus: Some(1),
                memory: Some("1gb".to_string()),
                storage: Some("1gb".to_string()),
                gpu: Some(GPUVar::Boolean(true)),
            }
        );
    }

    #[test]
    fn test_host_requirements_gpu_string() {
        use serde_json::json;

        let json = json!({
            "cpus": 1,
            "memory": "1gb",
            "storage": "1gb",
            "gpu": "optional"
        });
        let host_requirements: HostRequirements = serde_json::from_value(json).unwrap();
        assert_eq!(
            host_requirements,
            HostRequirements {
                cpus: Some(1),
                memory: Some("1gb".to_string()),
                storage: Some("1gb".to_string()),
                gpu: Some(GPUVar::String("optional".to_string())),
            }
        );
    }
}
