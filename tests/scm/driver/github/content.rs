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
use amp_common::scm::content::ContentService;
use amp_common::scm::driver::github::constants::GITHUB_PATH_CONTENTS;
use amp_common::scm::driver::github::content::GithubContentService;

#[tokio::test]
async fn test_find() {
    let repo = "octocat/Hello-World";
    let file = "README";

    let path = GITHUB_PATH_CONTENTS
        .replace("{repo}", repo)
        .replace("{file}", file);
    let setup = mock("GET", &path, "scm/github/contents/get-readme-success").await;

    let service = GithubContentService { client: setup.0 };
    let result = service.find(repo, file, "master").await;

    assert!(result.is_ok());
    let content = result.unwrap();

    assert_eq!(
        content.sha,
        "980a0d5f19a64b4b30a87d4206aade58726b60e3".to_string()
    );
    assert_eq!(content.data, "Hello World!\n".as_bytes());
}
