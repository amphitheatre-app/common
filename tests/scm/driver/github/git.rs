// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::common::mock;
use amp_common::scm::client::ListOptions;
use amp_common::scm::driver::github::constants::{
    GITHUB_PATH_BRANCHES, GITHUB_PATH_COMMITS, GITHUB_PATH_GIT_TREES, GITHUB_PATH_TAGS,
};
use amp_common::scm::driver::github::git::GithubGitService;
use amp_common::scm::git::GitService;

const REPO: &str = "octocat/Hello-World";
const REFERENCE: &str = "master";

#[test]
fn test_list_branches() {
    let path = GITHUB_PATH_BRANCHES.replace("{repo}", REPO);
    let setup = mock("GET", &path, "scm/github/git/list-branches-success");

    let service = GithubGitService { client: setup.0 };
    let result = service.list_branches(REPO, ListOptions::default());
    assert!(result.is_ok());
    assert!(result.unwrap().iter().any(|v| v.name.eq(&REFERENCE.to_string())));
}

#[test]
fn test_list_tags() {
    let path = GITHUB_PATH_TAGS.replace("{repo}", REPO);
    let setup = mock("GET", &path, "scm/github/git/list-tags-success");

    let service = GithubGitService { client: setup.0 };
    let result = service.list_tags(REPO, ListOptions::default());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

#[test]
fn test_find_commit() {
    let path = GITHUB_PATH_COMMITS
        .replace("{repo}", REPO)
        .replace("{reference}", REFERENCE);
    let setup = mock("GET", &path, "scm/github/git/find-commit-success");

    let service = GithubGitService { client: setup.0 };
    let result = service.find_commit(REPO, REFERENCE);
    assert!(result.is_ok());

    let commit = result.unwrap().unwrap();
    assert_eq!(commit.sha, "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d".to_string());
}

#[test]
fn test_git_trees() {
    let path = GITHUB_PATH_GIT_TREES
        .replace("{repo}", REPO)
        .replace("{tree_sha}", REFERENCE);

    let setup = mock("GET", &path, "scm/github/git/git-trees-success");
    let service = GithubGitService { client: setup.0 };
    let result = service.git_trees(REPO, REFERENCE, Some(true));
    assert!(result.is_ok());
}
