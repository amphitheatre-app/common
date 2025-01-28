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

use super::constants::{
    ATOMGIT_PATH_BRANCHES, ATOMGIT_PATH_COMMITS, ATOMGIT_PATH_GIT_TREES, ATOMGIT_PATH_TAGS,
};
use super::utils::convert_list_options;
use super::AtomGitFile;
use crate::http::{Client, Endpoint};
use crate::scm::client::ListOptions;
use crate::scm::git::{Commit, GitService, Reference, Signature, Tree, TreeEntry};
use crate::scm::utils;

pub struct AtomGitService {
    pub client: Client,
}

#[async_trait]
impl GitService for AtomGitService {
    /// Returns a list of branches for the specified repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-branch-list
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/branches
    async fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = ATOMGIT_PATH_BRANCHES.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<AtomGitBranchesEndpoint>(&path, options).await?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns a list of tags for the specified repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-tag-list
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/tags
    async fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = ATOMGIT_PATH_TAGS.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<AtomGitBranchesEndpoint>(&path, options).await?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns the contents of a single commit reference.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-ref-commit
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/commits/b851ec9b0129e1f744aa4a56b6c4bca0a5fce4b2
    async fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = ATOMGIT_PATH_COMMITS
            .replace("{repo}", repo)
            .replace("{reference}", reference);
        let res = self.client.get::<AtomGitCommitEndpoint>(&path, None).await?;

        Ok(res.data.map(|v| v.into()))
    }

    /// Returns a single tree using the SHA1 value or ref name for that tree.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repo-trees
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/trees/master
    async fn get_tree(
        &self,
        repo: &str,
        tree_sha: &str,
        recursive: Option<bool>,
    ) -> anyhow::Result<Option<Tree>> {
        let path = ATOMGIT_PATH_GIT_TREES
            .replace("{repo}", repo)
            .replace("{tree_sha}", tree_sha);
        let options = recursive
            .map(|r| Some(HashMap::from([("recursive".to_string(), r.to_string())])))
            .unwrap_or_default();
        let res = self.client.get::<AtomGitTreeEndpoint>(&path, options).await?;
        let option = res.data.unwrap();
        let tree = Tree {
            tree: option,
            ..Tree::default()
        };
        Ok(Option::from(tree))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitBranch {
    pub name: String,
    pub commit: AtomGitSimpleCommit,
    pub protected: bool,
}

impl From<&AtomGitBranch> for Reference {
    fn from(val: &AtomGitBranch) -> Self {
        Self {
            name: utils::trim_ref(&val.name),
            path: utils::expand_ref(&val.name, "refs/heads/"),
            sha: val.commit.sha.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitSimpleCommit {
    pub sha: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitCommit {
    pub sha: String,
    pub html_url: Option<String>,
    pub commit: AtomGitCommitObject,
    pub author: Option<AtomGitAuthor>,
    pub committer: Option<AtomGitAuthor>,
    pub files: Vec<AtomGitFile>,
}

impl From<AtomGitCommit> for Commit {
    fn from(val: AtomGitCommit) -> Self {
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
pub struct AtomGitCommitObject {
    pub author: AtomGitCommitObjectAuthor,
    pub committer: AtomGitCommitObjectAuthor,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitCommitObjectAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AtomGitAuthor {
    pub avatar_url: String,
    pub login: String,
}

struct AtomGitBranchesEndpoint;

impl Endpoint for AtomGitBranchesEndpoint {
    type Output = Vec<AtomGitBranch>;
}

struct AtomGitCommitEndpoint;

impl Endpoint for AtomGitCommitEndpoint {
    type Output = AtomGitCommit;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitTree {
    pub sha: String,
    pub tree: Vec<AtomGitTreeEntry>,
    pub truncated: bool,
}

impl From<AtomGitTree> for Tree {
    fn from(val: AtomGitTree) -> Self {
        Self {
            sha: val.sha,
            tree: val.tree.iter().map(|v| v.into()).collect(),
            truncated: val.truncated,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitTreeEntry {
    pub mode: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: Option<u64>,
}

impl From<&AtomGitTreeEntry> for TreeEntry {
    fn from(val: &AtomGitTreeEntry) -> Self {
        Self {
            mode: val.mode.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            kind: val.kind.clone(),
            size: val.size,
        }
    }
}

struct AtomGitTreeEndpoint;

impl Endpoint for AtomGitTreeEndpoint {
    type Output = Vec<TreeEntry>;
}
