// Copyright 2023 The Amphitheatre Authors.
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
use amp_common::scm::driver::gitlab::constants::{
    GITLAB_PATH_BRANCHES, GITLAB_PATH_COMMITS, GITLAB_PATH_TAGS,
};
use amp_common::scm::driver::gitlab::git::GitlabGitService;
use amp_common::scm::driver::gitlab::utils::encode;
use amp_common::scm::git::GitService;

const REPO: &str = "gitlab-org/gitlab-test";
const REFERENCE: &str = "master";

#[test]
fn test_list_branches() {
    let path = GITLAB_PATH_BRANCHES.replace("{repo}", &encode(REPO));
    let setup = mock("GET", &path, "scm/gitlab/git/list-branches-success");

    let service = GitlabGitService { client: setup.0 };
    let result = service.list_branches(REPO, ListOptions::default());
    assert!(result.is_ok());
    assert!(result
        .unwrap()
        .iter()
        .find(|v| v.name.eq(&"'test'".to_string()))
        .is_some());
}

#[test]
fn test_list_tags() {
    let path = GITLAB_PATH_TAGS.replace("{repo}", &encode(REPO));
    let setup = mock("GET", &path, "scm/gitlab/git/list-tags-success");

    let service = GitlabGitService { client: setup.0 };
    let result = service.list_tags(REPO, ListOptions::default());
    assert!(result.is_ok());
}

#[test]
fn test_find_commit() {
    let path = GITLAB_PATH_COMMITS
        .replace("{repo}", &encode(REPO))
        .replace("{reference}", REFERENCE);
    let setup = mock("GET", &path, "scm/gitlab/git/find-commit-success");

    let service = GitlabGitService { client: setup.0 };
    let result = service.find_commit(REPO, REFERENCE);
    assert!(result.is_ok());

    let commit = result.unwrap().unwrap();
    assert_eq!(commit.sha, "ddd0f15ae83993f5cb66a927a28673882e99100b".to_string());
}
