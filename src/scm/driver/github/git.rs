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

use super::GitHubFile;
use crate::client::{Client, Endpoint};
use crate::scm::git::{Commit, GitService, Reference, Signature};
use crate::scm::utils;

impl GitService for GitHubGitService {
    fn list_branches(&self, repo: &str) -> anyhow::Result<Vec<Reference>> {
        let path = format!("/repos/{}/branches", repo);
        let res = self.client.get::<GitHubBranchsEndpoint>(&path, None)?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    fn list_tags(&self, repo: &str) -> anyhow::Result<Vec<Reference>> {
        let path = format!("/repos/{}/tags", repo);
        let res = self.client.get::<GitHubBranchsEndpoint>(&path, None)?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = format!("/repos/{}/commits/{}", repo, reference);
        let res = self.client.get::<GitHubCommitEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubBranch {
    pub name: String,
    pub commit: GitHubSimpleCommit,
    pub protected: bool,
}

impl From<&GitHubBranch> for Reference {
    fn from(val: &GitHubBranch) -> Self {
        Self {
            name: utils::trim_ref(&val.name),
            path: utils::expand_ref(&val.name, "refs/heads/"),
            sha: val.commit.sha.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubSimpleCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubCommit {
    pub sha: String,
    pub url: String,
    pub commit: GitHubCommitObject,
    pub author: GitHubAuthor,
    pub committer: GitHubAuthor,
    pub files: Vec<GitHubFile>,
}

impl From<GitHubCommit> for Commit {
    fn from(val: GitHubCommit) -> Self {
        Self {
            sha: val.sha,
            message: val.commit.message,
            author: Signature {
                name: val.commit.author.name,
                email: val.commit.author.email,
                date: val.commit.author.date,
                login: Some(val.author.login),
                avatar: Some(val.author.avatar_url),
            },
            committer: Signature {
                name: val.commit.committer.name,
                email: val.commit.committer.email,
                date: val.commit.committer.date,
                login: Some(val.committer.login),
                avatar: Some(val.committer.avatar_url),
            },
            link: val.url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubCommitObject {
    pub author: GitHubCommitObjectAuthor,
    pub committer: GitHubCommitObjectAuthor,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubCommitObjectAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubAuthor {
    pub avatar_url: String,
    pub login: String,
}

struct GitHubBranchsEndpoint;

impl Endpoint for GitHubBranchsEndpoint {
    type Output = Vec<GitHubBranch>;
}

struct GitHubCommitEndpoint;

impl Endpoint for GitHubCommitEndpoint {
    type Output = GitHubCommit;
}

pub struct GitHubGitService {
    pub client: Client,
}

#[cfg(test)]
mod test {
    use super::GitHubGitService;
    use crate::client::Client;
    use crate::scm::git::GitService;

    #[test]
    fn test_list_branches() {
        let service = GitHubGitService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.list_branches("octocat/Hello-World");
        assert!(result.is_ok());
        assert!(result
            .unwrap()
            .iter()
            .find(|v| v.name.eq(&"master".to_string()))
            .is_some());
    }

    #[test]
    fn test_list_tags() {
        let service = GitHubGitService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.list_tags("octocat/Hello-World");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }

    #[test]
    fn test_find_commit() {
        let service = GitHubGitService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.find_commit("octocat/Hello-World", "master");
        assert!(result.is_ok());

        let commit = result.unwrap().unwrap();
        assert_eq!(commit.sha, "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d".to_string());
    }
}
