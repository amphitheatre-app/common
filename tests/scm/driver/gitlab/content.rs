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
use amp_common::scm::driver::gitlab::constants::GITLAB_PATH_CONTENTS;
use amp_common::scm::driver::gitlab::content::GitlabContentService;
use amp_common::scm::driver::gitlab::utils::{encode, encode_path};

#[tokio::test]
async fn test_find() {
    let repo = "gitlab-org/gitlab-test";
    let file = "VERSION";

    let path = GITLAB_PATH_CONTENTS
        .replace("{repo}", &encode(repo))
        .replace("{file}", &encode_path(file));
    let setup = mock("GET", &path, "scm/gitlab/contents/get-readme-success").await;

    let service = GitlabContentService { client: setup.0 };
    let result = service.find(repo, file, "master").await;

    assert!(result.is_ok());
    let content = result.unwrap();

    assert_eq!(
        content.sha,
        "913c66a37b4a45b9769037c55c2d238bd0942d2e".to_string()
    );
    assert_eq!(content.data, "6.7.0.pre\n".as_bytes());
}
