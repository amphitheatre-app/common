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

use amp_common::scm::{
    driver::gitlab::{constants::GITLAB_PATH_REPOS, repo::GitlabRepoService, utils::encode},
    repo::RepositoryService,
};

use crate::common::mock;

const REPO: &str = "octocat/Hello-World";
const REFERENCE: &str = "master";

#[test]
fn test_find() {
    let path = GITLAB_PATH_REPOS.replace("{repo}", &encode(REPO));
    let setup = mock("GET", &path, "scm/gitlab/repo/find-repo-success");

    let service = GitlabRepoService { client: setup.0 };
    let result = service.find(REPO);

    println!("{:?}", result);
    assert!(result.is_ok());

    let repo = result.unwrap().unwrap();
    assert_eq!(repo.branch, REFERENCE.to_string());
}
