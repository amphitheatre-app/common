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
