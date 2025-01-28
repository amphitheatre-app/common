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
use amp_common::scm::driver::atomgit::constants::{
    ATOMGIT_PATH_BRANCHES, ATOMGIT_PATH_COMMITS, ATOMGIT_PATH_GIT_TREES, ATOMGIT_PATH_TAGS,
};
use amp_common::scm::driver::atomgit::git::AtomGitService;
use amp_common::scm::git::GitService;

const REPO: &str = "jia-hao-li/atomgit_evaluation";
const REFERENCE: &str = "master";

#[tokio::test]
async fn test_list_branches() {
    let path = ATOMGIT_PATH_BRANCHES.replace("{repo}", REPO);
    let setup = mock("GET", &path, "scm/atomgit/git/list-branches-success").await;

    let service = AtomGitService { client: setup.0 };
    let result = service.list_branches(REPO, ListOptions::default()).await;

    assert!(result.is_ok());
    assert!(result.unwrap().iter().any(|v| v.name.eq(&REFERENCE.to_string())));
}

#[tokio::test]
async fn test_list_tags() {
    let path = ATOMGIT_PATH_TAGS.replace("{repo}", REPO);
    let setup = mock("GET", &path, "scm/atomgit/git/list-tags-success").await;

    let service = AtomGitService { client: setup.0 };
    let result = service.list_tags(REPO, ListOptions::default()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

#[tokio::test]
async fn test_find_commit() {
    let path = ATOMGIT_PATH_COMMITS
        .replace("{repo}", REPO)
        .replace("{reference}", REFERENCE);
    let setup = mock("GET", &path, "scm/atomgit/git/find-commit-success").await;

    let service = AtomGitService { client: setup.0 };
    let result = service.find_commit(REPO, REFERENCE).await;
    println!("{:?}", result);
    assert!(result.is_ok());

    let commit = result.unwrap().unwrap();
    assert_eq!(commit.sha, "b851ec9b0129e1f744aa4a56b6c4bca0a5fce4b2".to_string());
}

#[tokio::test]
async fn test_git_trees() {
    let path = ATOMGIT_PATH_GIT_TREES
        .replace("{repo}", REPO)
        .replace("{tree_sha}", REFERENCE);

    let setup = mock("GET", &path, "scm/atomgit/git/trees-success").await;
    let service = AtomGitService { client: setup.0 };
    let result = service.get_tree(REPO, REFERENCE, Some(true)).await;
    assert!(result.is_ok());
}
