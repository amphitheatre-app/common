// Copyright 2023 The Amphitheatre Authors.
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

use serde::{Deserialize, Serialize};

/// Your characters can depend on other partners from other registries,
/// git repositories, or subdirectories on your local file system.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Partner {
    /// Source code repository the package should be cloned from.
    /// e.g. https://github.com/amphitheatre-app/amphitheatre.git.
    pub repo: String,

    /// Git branch the package should be cloned from. eg. master or main
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// Git tag the package should be cloned from. eg. v1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// A commit hash like rev = "4c59b707", or a named reference exposed by
    /// the remote repository such as rev = "refs/pull/493/head". What references
    /// are available varies by where the repo is hosted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
}
