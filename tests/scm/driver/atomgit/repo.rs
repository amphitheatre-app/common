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

use amp_common::scm::driver::atomgit::constants::ATOMGIT_PATH_REPOS;
use amp_common::scm::driver::atomgit::repo::AtomGitRepoService;
use amp_common::scm::repo::RepositoryService;

use crate::common::mock;

const REPO: &str = "jia-hao-li/atomgit_evaluation";
const REFERENCE: &str = "master";
const DESCRIPTION: Option<&str> = Some("My first repository on AtomGit!");

#[test]
fn test_find() {
    let path = ATOMGIT_PATH_REPOS.replace("{repo}", REPO);
    let setup = mock("GET", &path, "scm/atomgit/repo/find-repo-success");

    let service = AtomGitRepoService { client: setup.0 };
    let result = service.find(REPO);

    println!("{:?}", result);
    assert!(result.is_ok());

    let repo = result.unwrap().unwrap();
    assert_eq!(repo.branch, REFERENCE.to_string());
    assert_eq!(
        repo.description.unwrap_or_default(),
        DESCRIPTION.unwrap_or_default()
    );
}
