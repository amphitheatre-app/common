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

/// represents a git repository.
#[derive(Debug)]
pub struct Content {
    pub path: String,
    pub data: Vec<u8>,
    pub sha: String,
    pub blob_id: String,
}

/// Provides access to repository content.
pub trait ContentService {
    /// Returns the repository file content by path.
    fn find(&self, repo: &str, path: &str, reference: &str) -> anyhow::Result<Content>;
}
