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

use super::constants::GITHUB_PATH_REPOS;

pub struct GithubRepoService {
    pub client: Client,
}

impl RepositoryService for GithubRepoService {
    /// Returns a repository by name.
    ///
    /// Docs: https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-a-repository
    /// Example: https://api.github.com/repos/octocat/Hello-World
    fn find(&self, repo: &str) -> anyhow::Result<Option<Repository>> {
        let path = GITHUB_PATH_REPOS.replace("{repo}", repo);
        let res = self.client.get::<GithubRepoEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubRepository {
    pub id: u64,
    pub name: String,
    pub owner: GithubOwner,
    pub html_url: String,
    pub archived: bool,
    pub visibility: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub default_branch: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubOwner {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
}

impl From<GithubRepository> for Repository {
    fn from(val: GithubRepository) -> Self {
        Self {
            id: val.id.to_string(),
            namespace: val.owner.login,
            name: val.name,
            branch: val.default_branch,
            archived: val.archived,
            visibility: val.visibility.into(),
            clone: val.clone_url,
            clone_ssh: val.ssh_url,
            link: val.html_url,
            created: val.created_at,
            updated: val.updated_at,
            description: val.description,
        }
    }
}

struct GithubRepoEndpoint;

impl Endpoint for GithubRepoEndpoint {
    type Output = GithubRepository;
}
