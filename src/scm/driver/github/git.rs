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
use std::collections::HashMap;

use super::constants::{GITHUB_PATH_BRANCHES, GITHUB_PATH_COMMITS, GITHUB_PATH_GIT_TREES, GITHUB_PATH_TAGS};
use super::utils::convert_list_options;
use super::GithubFile;
use crate::http::{Client, Endpoint};
use crate::scm::client::ListOptions;
use crate::scm::git::{Commit, GitService, Reference, Signature, Tree, TreeEntry};
use crate::scm::utils;

pub struct GithubGitService {
    pub client: Client,
}

#[async_trait]
impl GitService for GithubGitService {
    /// Returns a list of branches for the specified repository.
    ///
    /// Docs: https://docs.github.com/en/rest/branches/branches?apiVersion=2022-11-28#list-branches
    /// Example: https://api.github.com/repos/octocat/Hello-World/branches
    async fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = GITHUB_PATH_BRANCHES.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GithubBranchesEndpoint>(&path, options).await?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns a list of tags for the specified repository.
    ///
    /// Docs: https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-repository-tags
    /// Example: https://api.github.com/repos/octocat/Hello-World/tags
    async fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = GITHUB_PATH_TAGS.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GithubBranchesEndpoint>(&path, options).await?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns the contents of a single commit reference.
    ///
    /// Docs: https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#get-a-commit
    /// Example: https://api.github.com/repos/octocat/Hello-World/commits/master
    async fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = GITHUB_PATH_COMMITS
            .replace("{repo}", repo)
            .replace("{reference}", reference);
        let res = self.client.get::<GithubCommitEndpoint>(&path, None).await?;

        Ok(res.data.map(|v| v.into()))
    }

    /// Returns a single tree using the SHA1 value or ref name for that tree.
    ///
    /// Docs: https://docs.github.com/en/rest/git/trees?apiVersion=2022-11-28#get-a-tree
    /// Example: https://api.github.com/repos/octocat/Hello-World/git/trees/master
    async fn get_tree(
        &self,
        repo: &str,
        tree_sha: &str,
        recursive: Option<bool>,
    ) -> anyhow::Result<Option<Tree>> {
        let path = GITHUB_PATH_GIT_TREES
            .replace("{repo}", repo)
            .replace("{tree_sha}", tree_sha);
        let options = recursive
            .map(|r| Some(HashMap::from([("recursive".to_string(), r.to_string())])))
            .unwrap_or_default();
        let res = self.client.get::<GithubTreeEndpoint>(&path, options).await?;

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

struct GithubBranchesEndpoint;

impl Endpoint for GithubBranchesEndpoint {
    type Output = Vec<GithubBranch>;
}

struct GithubCommitEndpoint;

impl Endpoint for GithubCommitEndpoint {
    type Output = GithubCommit;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubTree {
    pub sha: String,
    pub tree: Vec<GithubTreeEntry>,
    pub truncated: bool,
}

impl From<GithubTree> for Tree {
    fn from(val: GithubTree) -> Self {
        Self {
            sha: val.sha,
            tree: val.tree.iter().map(|v| v.into()).collect(),
            truncated: val.truncated,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubTreeEntry {
    pub mode: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: Option<u64>,
}

impl From<&GithubTreeEntry> for TreeEntry {
    fn from(val: &GithubTreeEntry) -> Self {
        Self {
            mode: val.mode.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            kind: val.kind.clone(),
            size: val.size,
        }
    }
}

struct GithubTreeEndpoint;

impl Endpoint for GithubTreeEndpoint {
    type Output = GithubTree;
}
