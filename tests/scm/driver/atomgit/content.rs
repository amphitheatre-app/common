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
use amp_common::scm::driver::atomgit::constants::ATOMGIT_PATH_CONTENTS;
use amp_common::scm::driver::atomgit::content::AtomGitContentService;

#[test]
fn test_find() {
    let repo = "jia-hao-li/atomgit_evaluation";
    let file = "README";

    let path = ATOMGIT_PATH_CONTENTS
        .replace("{repo}", repo)
        .replace("{file}", file);
    let setup = mock("GET", &path, "scm/atomgit/contents/get-readme-success");

    let service = AtomGitContentService { client: setup.0 };
    let result = service.find(repo, file, "master");

    assert!(result.is_ok());
    let content = result.unwrap();

    assert_eq!(
        content.sha,
        "0c4db19796e8cd1abc958c408411c978feac693b".to_string()
    );
    //assert_eq!(content.data, "Hello World!\n".as_bytes());
}
