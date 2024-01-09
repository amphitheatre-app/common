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

/// A mount can be a bind mount or a volume mount.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrMount {
    String(String),
    Mount(Mount),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    /// Mount type.
    #[serde(rename = "type")]
    pub kind: MountKind,
    /// Mount source.
    pub source: String,
    /// Mount target.
    pub target: String,
}

/// Mount type.
#[derive(Debug, Serialize, Deserialize)]
pub enum MountKind {
    Bind,
    Volume,
}
