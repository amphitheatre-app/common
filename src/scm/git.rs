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

use super::{client::ListOptions, errors::SCMError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents a git reference.
#[derive(Debug, PartialEq)]
pub struct Reference {
    pub name: String,
    pub path: String,
    pub sha: String,
}

/// Represents a repository commit.
#[derive(Debug, Default, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: Signature,
    pub committer: Signature,
    pub link: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub date: String,
    // Fields are optional. The provider may choose to
    // include account information in the response.
    pub login: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Tree {
    pub sha: String,
    pub tree: Vec<TreeEntry>,
    pub truncated: bool,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TreeEntry {
    pub mode: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: Option<u64>,
}

/// Provides access to git resources.
#[async_trait]
pub trait GitService: Send + Sync {
    /// Returns a list of git branches.
    async fn list_branches(&self, repo: &str, opts: ListOptions) -> Result<Vec<Reference>, SCMError>;

    /// Returns a list of git tags.
    async fn list_tags(&self, repo: &str, opts: ListOptions) -> Result<Vec<Reference>, SCMError>;

    /// Finds a git commit by reference
    async fn find_commit(&self, repo: &str, reference: &str) -> Result<Option<Commit>, SCMError>;

    /// Returns a single tree using the SHA1 value or ref name for that tree.
    async fn get_tree(
        &self,
        repo: &str,
        sha: &str,
        recursive: Option<bool>,
    ) -> Result<Option<Tree>, SCMError>;
}
