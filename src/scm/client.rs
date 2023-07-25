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

use super::content::ContentService;
use super::driver::{Driver, DriverTrait};
use super::errors::SCMError;
use super::git::GitService;
use super::repo::RepositoryService;
use crate::config::Credentials;

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

pub struct Client {
    driver: Driver,
}

impl Client {
    pub fn new(driver: Driver) -> Self {
        Self { driver }
    }

    pub fn contents(&self) -> Box<dyn ContentService> {
        self.driver.contents()
    }

    pub fn git(&self) -> Box<dyn GitService> {
        self.driver.git()
    }

    pub fn repositories(&self) -> Box<dyn RepositoryService> {
        self.driver.repositories()
    }
}

impl Client {
    /// Initialize the client by source repostory.
    pub fn init(credentials: &Credentials, repo: &str) -> Result<Client, SCMError> {
        if let Some(repo) = credentials.find_repository(repo) {
            return Ok(Self::new(Driver::try_from(repo)?));
        }

        Ok(Client::new(Driver::try_from(repo)?))
    }
}

#[cfg(test)]
mod test {
    use crate::scm::client::Client;
    use crate::scm::driver::github;

    #[test]
    fn call_content_service() {
        let client = Client::new(github::default());
        let content = client.contents().find("octocat/Hello-World", "README", "master");

        assert!(content.is_ok());
    }

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
    }
}
