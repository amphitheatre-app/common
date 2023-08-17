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

pub mod constants;
pub mod content;
pub mod driver;
pub mod git;
pub mod pr;
pub mod repo;
mod utils;

use self::constants::GITHUB_ENDPOINT;
use self::driver::GithubDriver;
use self::pr::GithubFile;
use super::Driver;
use crate::http::Client;

/// Returns a new GitHub driver using the default api.github.com address.
#[inline]
pub fn default() -> Driver {
    from(Client::new(GITHUB_ENDPOINT, None))
}

/// Returnes a new GitHub driver.
#[inline]
pub fn new(url: &str, token: Option<String>) -> Driver {
    from(Client::new(url, token))
}

/// Returns a new GitHub driver using the given client.
pub fn from(client: Client) -> Driver {
    Driver::Github(GithubDriver { client })
}

#[cfg(test)]
mod test {
    use super::constants::GITHUB_ENDPOINT;
    use crate::{http::Client, scm::driver::github};

    #[test]
    fn create_github_driver() {
        let _driver = github::default();
    }

    #[test]
    fn create_github_driver_from_client() {
        let _driver = github::from(Client::new(GITHUB_ENDPOINT, None));
    }

    #[test]
    fn create_github_enterprise_driver() {
        let _driver = github::new("https://github.company.com/api/v3", None);
    }
}
