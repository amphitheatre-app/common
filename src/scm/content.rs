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

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::errors::SCMError;

/// represents a file content in a repository.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Content {
    pub path: String,
    pub data: Vec<u8>,
    pub sha: String,
    pub blob_id: String,
}

/// represents a file in a repository.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct File {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub blob_id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

/// Provides access to repository content.
#[async_trait]
pub trait ContentService: Send + Sync {
    /// Returns the repository file content by path.
    async fn find(&self, repo: &str, path: &str, reference: &str) -> Result<Content, SCMError>;

    /// Returns the file list in a repository folder.
    async fn list(&self, repo: &str, path: &str, reference: &str) -> Result<Vec<File>, SCMError>;
}
