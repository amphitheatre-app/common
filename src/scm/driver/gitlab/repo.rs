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

use serde::{Deserialize, Serialize};

use super::utils::encode;
use crate::http::{Client, Endpoint};
use crate::scm::constants::Visibility;
use crate::scm::repo::{Repository, RepositoryService};

pub struct GitlabRepoService {
    pub client: Client,
}

impl RepositoryService for GitlabRepoService {
    /// Returns a repository by name.
    fn find(&self, repo: &str) -> anyhow::Result<Option<Repository>> {
        let path = format!("/api/v4/projects/{}", encode(repo));
        let res = self.client.get::<GitlabRepoEndpoint>(&path, None)?;

        println!("{:?}", res);

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitlabRepository {
    pub id: u64,
    pub path: String,
    pub path_with_namespace: String,
    pub default_branch: String,
    // pub visibility: String,
    // pub archived: bool,
    pub web_url: String,
    #[serde(rename = "ssh_url_to_repo")]
    pub ssh_url: String,
    #[serde(rename = "http_url_to_repo")]
    pub http_url: String,
    pub namespace: GitlabNamespace,
    pub created_at: String,
    // pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitlabNamespace {
    pub name: String,
    pub path: String,
    pub full_path: String,
}

impl From<GitlabRepository> for Repository {
    fn from(val: GitlabRepository) -> Self {
        let mut to = Self {
            id: val.id.to_string(),
            namespace: val.namespace.path,
            name: val.path,
            branch: val.default_branch,
            visibility: Visibility::Unknown,
            archived: false,
            clone: val.http_url,
            clone_ssh: val.ssh_url,
            link: val.web_url,
            created: val.created_at,
            updated: "".to_string(),
        };

        if !val.namespace.full_path.is_empty() {
            to.namespace = val.namespace.full_path;
        }

        if to.namespace.is_empty() {
            let parts: Vec<&str> = val.path_with_namespace.splitn(2, '/').collect();
            if parts.len() == 2 {
                to.namespace = parts[1].to_string();
            }
        }

        to
    }
}

struct GitlabRepoEndpoint;

impl Endpoint for GitlabRepoEndpoint {
    type Output = GitlabRepository;
}

#[cfg(test)]
mod test {
    use super::GitlabRepoService;
    use crate::http::Client;
    use crate::scm::repo::RepositoryService;

    #[test]
    fn test_find() {
        let service = GitlabRepoService {
            client: Client::new("https://gitlab.com", None),
        };

        let result = service.find("gitlab-org/gitlab-test");

        println!("{:?}", result);
        assert!(result.is_ok());

        let repo = result.unwrap().unwrap();
        assert_eq!(repo.branch, "master".to_string());
    }
}
