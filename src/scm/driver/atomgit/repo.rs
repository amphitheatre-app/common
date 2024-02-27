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

use crate::http::{Client, Endpoint};
use crate::scm::repo::{Repository, RepositoryService};

use super::constants::ATOMGIT_PATH_REPOS;

pub struct AtomGitRepoService {
    pub client: Client,
}

impl RepositoryService for AtomGitRepoService {
    /// Returns a repository by name.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repository/
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation
    fn find(&self, repo: &str) -> anyhow::Result<Option<Repository>> {
        let path = ATOMGIT_PATH_REPOS.replace("{repo}", repo);
        let res = self.client.get::<AtomGitRepoEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitRepository {
    pub id: u64,
    pub name: String,
    pub owner: Option<AtomGitOwner>,
    pub html_url: String,
    pub archived: bool,
    pub visibility: String,
    pub clone_url: Option<String>,
    pub ssh_url: Option<String>,
    pub default_branch: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AtomGitOwner {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
}

impl From<AtomGitRepository> for Repository {
    fn from(val: AtomGitRepository) -> Self {
        Self {
            id: val.id.to_string(),
            namespace: val.owner.unwrap_or_default().login,
            name: val.name,
            branch: val.default_branch,
            archived: val.archived,
            visibility: val.visibility.into(),
            clone: val.clone_url.unwrap_or_default(),
            clone_ssh: val.ssh_url.unwrap_or_default(),
            link: val.html_url,
            created: val.created_at.unwrap_or_default(),
            updated: val.updated_at.unwrap_or_default(),
            description: val.description,
        }
    }
}

struct AtomGitRepoEndpoint;

impl Endpoint for AtomGitRepoEndpoint {
    type Output = AtomGitRepository;
}
