// Copyright 2023 The Amphitheatre Authors.
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

use super::content::GitlabContentService;
use super::git::GitlabGitService;
use super::repo::GitlabRepoService;
use crate::scm::driver::Driver;

pub struct GitlabDriver {
    pub client: crate::client::Client,
}

impl Driver for GitlabDriver {
    fn contents(&self) -> impl crate::scm::content::ContentService {
        GitlabContentService {
            client: self.client.clone(),
        }
    }

    fn git(&self) -> impl crate::scm::git::GitService {
        GitlabGitService {
            client: self.client.clone(),
        }
    }

    fn repositories(&self) -> impl crate::scm::repo::RepositoryService {
        GitlabRepoService {
            client: self.client.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::scm::driver::{gitlab, Driver};

    #[test]
    fn return_git_service() {
        gitlab::default().git();
    }

    #[test]
    fn return_repo_service() {
        gitlab::default().repositories();
    }
}
