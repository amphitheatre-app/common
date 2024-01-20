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

use super::constants::{GITLAB_PATH_BRANCHES, GITLAB_PATH_COMMITS, GITLAB_PATH_TAGS};
use super::utils::{convert_list_options, encode};
use crate::http::{Client, Endpoint};
use crate::scm::client::ListOptions;
use crate::scm::git::{Commit, GitService, Reference, Signature, Tree};
use crate::scm::utils;

pub struct GitlabGitService {
    pub client: Client,
}

impl GitService for GitlabGitService {
    /// List repository branches.
    ///
    /// Docs: https://docs.gitlab.com/ee/api/branches.html#list-repository-branches
    /// Example: https://gitlab.com/api/v4/projects/gitlab-org%2Fgitlab-test/repository/branches
    fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = GITLAB_PATH_BRANCHES.replace("{repo}", &encode(repo));
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GitlabBranchesEndpoint>(&path, options)?;

        if let Some(branches) = res.data {
            return Ok(branches.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// List project repository tags.
    ///
    /// Docs: https://docs.gitlab.com/ee/api/tags.html#list-project-repository-tags
    /// Example: https://gitlab.com/api/v4/projects/gitlab-org%2Fgitlab-test/repository/tags
    fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        let path = GITLAB_PATH_TAGS.replace("{repo}", &encode(repo));
        let options = Some(convert_list_options(opts));
        let res = self.client.get::<GitlabBranchesEndpoint>(&path, options)?;

        if let Some(tags) = res.data {
            return Ok(tags.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }

    /// Get a single commit.
    ///
    /// Docs: https://docs.gitlab.com/ee/api/commits.html#get-a-single-commit
    /// Example: https://gitlab.com/api/v4/projects/gitlab-org%2Fgitlab-test/repository/commits/master
    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        let path = GITLAB_PATH_COMMITS
            .replace("{repo}", &encode(repo))
            .replace("{reference}", reference);
        let res = self.client.get::<GitlabCommitEndpoint>(&path, None)?;

        Ok(res.data.map(|v| v.into()))
    }

    /// @TODO: Get a tree of a project.
    fn get_tree(
        &self,
        _repo: &str,
        _tree_sha: &str,
        _recursive: Option<bool>,
    ) -> anyhow::Result<Option<Tree>> {
        todo!()
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

struct GitlabBranchesEndpoint;

impl Endpoint for GitlabBranchesEndpoint {
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
