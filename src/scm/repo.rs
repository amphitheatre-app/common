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

use super::constants::Visibility;

/// represents a git repository.
#[derive(Debug)]
pub struct Repository {
    pub id: String,
    pub namespace: String,
    pub name: String,
    pub branch: String,
    pub archived: bool,
    pub visibility: Visibility,
    pub clone: String,
    pub clone_ssh: String,
    pub link: String,
    pub created: String,
    pub updated: String,
}

/// Provides access to repository resources.
pub trait RepositoryService {
    /// Returns a repository by name.
    fn find(&self, repo: &str) -> anyhow::Result<Option<Repository>>;
}
