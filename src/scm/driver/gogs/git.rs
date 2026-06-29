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

use super::constants::{GOGS_PATH_BRANCHES, GOGS_PATH_COMMITS, GOGS_PATH_GIT_TREES, GOGS_PATH_TAGS};
use super::utils::convert_list_options;
use super::GogsFile;
use crate::http::{endpoint::Endpoint, Client};
use crate::scm::client::ListOptions;
use crate::scm::errors::SCMError;
use crate::scm::git::{Commit, GitService, Reference, Signature, Tree, TreeEntry};
use crate::scm::utils;

pub struct GogsService {
    pub client: Client,
}

#[async_trait]
impl GitService for GogsService {
    /// Returns a list of branches for the specified repository.
    ///
    /// Docs: https://gogs.io/docs/api
    /// Example: https://gogs.io/api/v1/repos/gogs/gogs/branches
    async fn list_branches(&self, repo: &str, opts: ListOptions) -> Result<Vec<Reference>, SCMError> {
        let path = GOGS_PATH_BRANCHES.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self
            .client
            .get::<Vec<GogsBranch>>(&path, options)
            .await
            .map_err(SCMError::ClientError)?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns a list of tags for the specified repository.
    ///
    /// Docs: https://gogs.io/docs/api
    /// Example: https://gogs.io/api/v1/repos/gogs/gogs/tags
    async fn list_tags(&self, repo: &str, opts: ListOptions) -> Result<Vec<Reference>, SCMError> {
        let path = GOGS_PATH_TAGS.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self
            .client
            .get::<Vec<GogsBranch>>(&path, options)
            .await
            .map_err(SCMError::ClientError)?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns the contents of a single commit reference.
    ///
    /// Docs: https://gogs.io/docs/api
    /// Example: https://gogs.io/api/v1/repos/gogs/gogs/git/commits/4b28b80e1c5f8c0c5c7c5c5f5c5c5c5c5c5c5c5c
    async fn find_commit(&self, repo: &str, reference: &str) -> Result<Option<Commit>, SCMError> {
        let path = GOGS_PATH_COMMITS
            .replace("{repo}", repo)
            .replace("{reference}", reference);
        let res = self
            .client
            .get::<GogsCommit>(&path, None)
            .await
            .map_err(SCMError::ClientError)?;

        Ok(res.data.map(|v| v.into()))
    }

    /// Returns a single tree using the SHA1 value or ref name for that tree.
    ///
    /// Docs: https://gogs.io/docs/api
    /// Example: https://gogs.io/api/v1/repos/gogs/gogs/git/trees/master
    async fn get_tree(
        &self,
        repo: &str,
        tree_sha: &str,
        recursive: Option<bool>,
    ) -> Result<Option<Tree>, SCMError> {
        let path = GOGS_PATH_GIT_TREES
            .replace("{repo}", repo)
            .replace("{tree_sha}", tree_sha);
        let options = recursive
            .map(|r| Some(HashMap::from([("recursive".to_string(), r.to_string())])))
            .unwrap_or_default();
        let res = self
            .client
            .get::<GogsTree>(&path, options)
            .await
            .map_err(SCMError::ClientError)?;

        Ok(res.data.map(|v| v.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsBranch {
    pub name: String,
    pub commit: GogsSimpleCommit,
    pub protected: bool,
}

impl From<&GogsBranch> for Reference {
    fn from(val: &GogsBranch) -> Self {
        Self {
            name: utils::trim_ref(&val.name),
            path: utils::expand_ref(&val.name, "refs/heads/"),
            sha: val.commit.sha.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsSimpleCommit {
    // Gogs uses "id" for the commit sha in some payloads, and "sha" in others,
    // so accept both.
    #[serde(alias = "id")]
    pub sha: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsCommit {
    pub sha: String,
    pub html_url: Option<String>,
    pub commit: GogsCommitObject,
    pub author: Option<GogsAuthor>,
    pub committer: Option<GogsAuthor>,
    pub files: Vec<GogsFile>,
}

impl From<GogsCommit> for Commit {
    fn from(val: GogsCommit) -> Self {
        Self {
            sha: val.sha,
            message: val.commit.message.unwrap_or_default(),
            author: Signature {
                name: val.commit.author.name,
                email: val.commit.author.email,
                date: val.commit.author.date,
                login: Some(val.author.clone().unwrap_or_default().login),
                avatar: Some(val.author.clone().unwrap_or_default().avatar_url),
            },
            committer: Signature {
                name: val.commit.committer.name,
                email: val.commit.committer.email,
                date: val.commit.committer.date,
                login: Some(val.committer.clone().unwrap_or_default().login),
                avatar: Some(val.committer.clone().unwrap_or_default().avatar_url),
            },
            link: val.html_url.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsCommitObject {
    pub author: GogsCommitObjectAuthor,
    pub committer: GogsCommitObjectAuthor,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsCommitObjectAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct GogsAuthor {
    pub avatar_url: String,
    pub login: String,
}

impl Endpoint for Vec<GogsBranch> {
    type Output = Vec<GogsBranch>;
}

impl Endpoint for GogsCommit {
    type Output = GogsCommit;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsTree {
    pub sha: String,
    pub tree: Vec<GogsTreeEntry>,
    pub truncated: bool,
}

impl From<GogsTree> for Tree {
    fn from(val: GogsTree) -> Self {
        Self {
            sha: val.sha,
            tree: val.tree.iter().map(|v| v.into()).collect(),
            truncated: val.truncated,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsTreeEntry {
    pub mode: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: Option<u64>,
}

impl From<&GogsTreeEntry> for TreeEntry {
    fn from(val: &GogsTreeEntry) -> Self {
        Self {
            mode: val.mode.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            kind: val.kind.clone(),
            size: val.size,
        }
    }
}

impl Endpoint for GogsTree {
    type Output = GogsTree;
}
