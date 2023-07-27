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

use super::utils::{convert_list_options, encode};
use crate::http::{Client, Endpoint};
use crate::scm::client::ListOptions;
use crate::scm::git::{Commit, GitService, Reference, Signature};
use crate::scm::utils;

pub struct GitlabGitService {
    pub client: Client,
}

impl GitService for GitlabGitService {
    fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = format!("/api/v4/projects/{}/repository/branches", encode(repo));
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GitlabBranchsEndpoint>(&path, options)?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = format!("/api/v4/projects/{}/repository/tags", encode(repo));
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GitlabBranchsEndpoint>(&path, options)?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = format!(
            "/api/v4/projects/{}/repository/commits/{}",
            encode(repo),
            reference
        );
        let res = self.client.get::<GitlabCommitEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitlabBranch {
    pub name: String,
    pub commit: GitlabCommit,
}

impl From<&GitlabBranch> for Reference {
    fn from(val: &GitlabBranch) -> Self {
        Self {
            name: utils::trim_ref(&val.name),
            path: utils::expand_ref(&val.name, "refs/heads/"),
            sha: val.commit.id.clone(),
        }
    }
}

struct GitlabBranchsEndpoint;

impl Endpoint for GitlabBranchsEndpoint {
    type Output = Vec<GitlabBranch>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitlabCommit {
    pub id: String,
    pub title: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_date: String,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_date: String,
    pub created_at: String,
    pub web_url: String,
}

impl From<GitlabCommit> for Commit {
    fn from(val: GitlabCommit) -> Self {
        Self {
            sha: val.id,
            message: val.message,
            author: Signature {
                name: val.author_name.clone(),
                email: val.author_email,
                date: val.authored_date,
                login: Some(val.author_name),
                avatar: None,
            },
            committer: Signature {
                name: val.committer_name.clone(),
                email: val.committer_email,
                date: val.committed_date,
                login: Some(val.committer_name),
                avatar: None,
            },
            link: val.web_url,
        }
    }
}
struct GitlabCommitEndpoint;

impl Endpoint for GitlabCommitEndpoint {
    type Output = GitlabCommit;
}

#[cfg(test)]
mod test {
    use crate::http::Client;
    use crate::scm::client::ListOptions;
    use crate::scm::driver::gitlab::git::GitlabGitService;
    use crate::scm::git::GitService;

    #[test]
    fn test_list_branches() {
        let service = GitlabGitService {
            client: Client::new("https://gitlab.com", None),
        };
        let result = service.list_branches("gitlab-org/gitlab-test", ListOptions::default());
        println!("{:?}", result);
        assert!(result.is_ok());
        assert!(result
            .unwrap()
            .iter()
            .find(|v| v.name.eq(&"'test'".to_string()))
            .is_some());
    }

    #[test]
    fn test_list_tags() {
        let service = GitlabGitService {
            client: Client::new("https://gitlab.com", None),
        };
        let result = service.list_branches("gitlab-org/gitlab-test", ListOptions::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_commit() {
        let service = GitlabGitService {
            client: Client::new("https://gitlab.com", None),
        };
        let result = service.find_commit("gitlab-org/gitlab-test", "master");
        assert!(result.is_ok());

        let commit = result.unwrap().unwrap();
        assert_eq!(commit.sha, "ddd0f15ae83993f5cb66a927a28673882e99100b".to_string());
    }
}
