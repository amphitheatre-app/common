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
use std::collections::HashMap;

use super::constants::{
    ATOMGIT_PATH_BRANCHES, ATOMGIT_PATH_COMMITS, ATOMGIT_PATH_GIT_TREES, ATOMGIT_PATH_TAGS,
};
use super::utils::convert_list_options;
use super::AtomgitFile;
use crate::http::{Client, Endpoint};
use crate::scm::client::ListOptions;
use crate::scm::git::{Commit, GitService, Reference, Signature, Tree, TreeEntry};
use crate::scm::utils;

pub struct AtomGitService {
    pub client: Client,
}

impl GitService for AtomGitService {
    /// Returns a list of branches for the specified repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-branch-list
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/branches
    fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = ATOMGIT_PATH_BRANCHES.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<AtomgitBranchesEndpoint>(&path, options)?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns a list of tags for the specified repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-tag-list
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/tags
    fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = ATOMGIT_PATH_TAGS.replace("{repo}", repo);
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<AtomgitBranchesEndpoint>(&path, options)?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Returns the contents of a single commit reference.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-ref-commit
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/commits/b851ec9b0129e1f744aa4a56b6c4bca0a5fce4b2
    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = ATOMGIT_PATH_COMMITS
            .replace("{repo}", repo)
            .replace("{reference}", reference);
        let res = self.client.get::<AtomgitCommitEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }

    /// Returns a single tree using the SHA1 value or ref name for that tree.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repo-trees
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/trees/master
    fn get_tree(&self, repo: &str, tree_sha: &str, recursive: Option<bool>) -> anyhow::Result<Option<Tree>> {
        let path = ATOMGIT_PATH_GIT_TREES
            .replace("{repo}", repo)
            .replace("{tree_sha}", tree_sha);
        let options = recursive
            .map(|r| Some(HashMap::from([("recursive".to_string(), r.to_string())])))
            .unwrap_or_default();
        let res = self.client.get::<AtomgitTreeEndpoint>(&path, options)?;
        let option = res.data.unwrap();
        let tree = Tree {
            tree: option,
            ..Tree::default()
        };
        Ok(Option::from(tree))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitBranch {
    pub name: String,
    pub commit: AtomgitSimpleCommit,
    pub protected: bool,
}

impl From<&AtomgitBranch> for Reference {
    fn from(val: &AtomgitBranch) -> Self {
        Self {
            name: utils::trim_ref(&val.name),
            path: utils::expand_ref(&val.name, "refs/heads/"),
            sha: val.commit.sha.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitSimpleCommit {
    pub sha: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitCommit {
    pub sha: String,
    pub html_url: Option<String>,
    pub commit: AtomgitCommitObject,
    pub author: AtomgitAuthor,
    pub committer: AtomgitAuthor,
    pub files: Vec<AtomgitFile>,
}

impl From<AtomgitCommit> for Commit {
    fn from(val: AtomgitCommit) -> Self {
        Self {
            sha: val.sha,
            message: val.commit.message.unwrap(),
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
            link: val.html_url.unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitCommitObject {
    pub author: AtomgitCommitObjectAuthor,
    pub committer: AtomgitCommitObjectAuthor,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitCommitObjectAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitAuthor {
    pub avatar_url: String,
    pub login: String,
}

struct AtomgitBranchesEndpoint;

impl Endpoint for AtomgitBranchesEndpoint {
    type Output = Vec<AtomgitBranch>;
}

struct AtomgitCommitEndpoint;

impl Endpoint for AtomgitCommitEndpoint {
    type Output = AtomgitCommit;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitTree {
    pub sha: String,
    pub tree: Vec<AtomgitTreeEntry>,
    pub truncated: bool,
}

impl From<AtomgitTree> for Tree {
    fn from(val: AtomgitTree) -> Self {
        Self {
            sha: val.sha,
            tree: val.tree.iter().map(|v| v.into()).collect(),
            truncated: val.truncated,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitTreeEntry {
    pub mode: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: Option<u64>,
}

impl From<&AtomgitTreeEntry> for TreeEntry {
    fn from(val: &AtomgitTreeEntry) -> Self {
        Self {
            mode: val.mode.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            kind: val.kind.clone(),
            size: val.size,
        }
    }
}

struct AtomgitTreeEndpoint;

impl Endpoint for AtomgitTreeEndpoint {
    type Output = Vec<TreeEntry>;
}
