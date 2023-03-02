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

use super::utils::convert_list_options;
use super::GithubFile;
use crate::client::{Client, Endpoint};
use crate::scm::client::ListOptions;
use crate::scm::git::{Commit, GitService, Reference, Signature};
use crate::scm::utils;

pub struct GithubGitService {
    pub client: Client,
}

impl GitService for GithubGitService {
    fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = format!("/repos/{}/branches", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GithubBranchsEndpoint>(&path, options)?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = format!("/repos/{}/tags", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GithubBranchsEndpoint>(&path, options)?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = format!("/repos/{}/commits/{}", repo, reference);
        let res = self.client.get::<GithubCommitEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubBranch {
    pub name: String,
    pub commit: GithubSimpleCommit,
    pub protected: bool,
}

impl From<&GithubBranch> for Reference {
    fn from(val: &GithubBranch) -> Self {
        Self {
            name: utils::trim_ref(&val.name),
            path: utils::expand_ref(&val.name, "refs/heads/"),
            sha: val.commit.sha.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubSimpleCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubCommit {
    pub sha: String,
    pub html_url: String,
    pub commit: GithubCommitObject,
    pub author: GithubAuthor,
    pub committer: GithubAuthor,
    pub files: Vec<GithubFile>,
}

impl From<GithubCommit> for Commit {
    fn from(val: GithubCommit) -> Self {
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
            link: val.html_url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubCommitObject {
    pub author: GithubCommitObjectAuthor,
    pub committer: GithubCommitObjectAuthor,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubCommitObjectAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubAuthor {
    pub avatar_url: String,
    pub login: String,
}

struct GithubBranchsEndpoint;

impl Endpoint for GithubBranchsEndpoint {
    type Output = Vec<GithubBranch>;
}

struct GithubCommitEndpoint;

impl Endpoint for GithubCommitEndpoint {
    type Output = GithubCommit;
}

#[cfg(test)]
mod test {
    use super::GithubGitService;
    use crate::client::Client;
    use crate::scm::client::ListOptions;
    use crate::scm::git::GitService;

    #[test]
    fn test_list_branches() {
        let service = GithubGitService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.list_branches("octocat/Hello-World", ListOptions::default());
        assert!(result.is_ok());
        assert!(result
            .unwrap()
            .iter()
            .find(|v| v.name.eq(&"master".to_string()))
            .is_some());
    }

    #[test]
    fn test_list_tags() {
        let service = GithubGitService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.list_tags("octocat/Hello-World", ListOptions::default());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }

    #[test]
    fn test_find_commit() {
        let service = GithubGitService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.find_commit("octocat/Hello-World", "master");
        assert!(result.is_ok());

        let commit = result.unwrap().unwrap();
        assert_eq!(commit.sha, "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d".to_string());
    }
}
