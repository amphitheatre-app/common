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

use super::content::AtomGitContentService;
use super::git::AtomGitService;
use super::repo::AtomGitRepoService;
use crate::http::Client;
use crate::scm::content::ContentService;
use crate::scm::driver::DriverTrait;
use crate::scm::git::GitService;
use crate::scm::repo::RepositoryService;

pub struct AtomGitDriver {
    pub client: Client,
}

impl DriverTrait for AtomGitDriver {
    fn contents(&self) -> Box<dyn ContentService> {
        Box::new(AtomGitContentService {
            client: self.client.clone(),
        })
    }

    fn git(&self) -> Box<dyn GitService> {
        Box::new(AtomGitService {
            client: self.client.clone(),
        })
    }

    fn repositories(&self) -> Box<dyn RepositoryService> {
        Box::new(AtomGitRepoService {
            client: self.client.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::scm::driver::{atomgit, DriverTrait};

    #[test]
    fn return_git_service() {
        atomgit::default().git();
    }

    #[test]
    fn return_repo_service() {
        atomgit::default().repositories();
    }
}
