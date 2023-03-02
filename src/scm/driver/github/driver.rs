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

use super::git::GitHubGitService;
use super::repo::GitHubRepoService;
use crate::client::Client;
use crate::scm::driver::Driver;
use crate::scm::git::GitService;
use crate::scm::repo::RepositoryService;

pub struct GithubDriver {
    pub client: Client,
}

impl Driver for GithubDriver {
    fn git(&self) -> impl GitService {
        GitHubGitService {
            client: self.client.clone(),
        }
    }

    fn repositories(&self) -> impl RepositoryService {
        GitHubRepoService {
            client: self.client.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::scm::driver::{github, Driver};

    #[test]
    fn return_git_service() {
        github::default().git();
    }

    #[test]
    fn return_repo_service() {
        github::default().repositories();
    }
}
