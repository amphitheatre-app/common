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

use super::driver::Driver;
use super::git::GitService;
use super::repo::RepositoryService;

/// Specifies optional pagination
pub struct ListOptions {
    pub url: Option<String>,
    pub page: i32,
    pub size: i32,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            url: None,
            page: 1,
            size: 30,
        }
    }
}

pub struct Client<T: Driver> {
    driver: T,
}

impl<T: Driver> Client<T> {
    pub fn new(driver: T) -> Self {
        Self { driver }
    }

    pub fn git(&self) -> impl GitService {
        self.driver.git()
    }

    pub fn repositories(&self) -> impl RepositoryService {
        self.driver.repositories()
    }
}

#[cfg(test)]
mod test {
    use crate::scm::client::Client;
    use crate::scm::driver::github;
    use crate::scm::git::GitService;
    use crate::scm::repo::RepositoryService;

    #[test]
    fn call_get_service() {
        let client = Client::new(github::default());
        let commit = client.git().find_commit("octocat/Hello-World", "master");

        assert!(commit.is_ok());
    }

    #[test]
    fn call_repo_service() {
        let client = Client::new(github::default());
        let repo = client.repositories().find("octocat/Hello-World");

        assert!(repo.is_ok());
        assert_eq!(&repo.unwrap().unwrap().branch, "master")
    }
}
