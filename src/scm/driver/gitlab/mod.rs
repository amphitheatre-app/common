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

use self::constants::GITLAB_ENDPOINT;
use self::driver::GitlabDriver;
use super::Driver;
use crate::http::Client;

pub mod constants;
pub mod content;
pub mod driver;
pub mod git;
pub mod repo;
pub mod utils;

/// Returns a new Gitlab driver using the default gitlab.com address.
pub fn default() -> Driver {
    from(Client::new(GITLAB_ENDPOINT, None).expect("Failed to create Gitlab client"))
}

/// Returns a new Gitlab driver.
pub fn new(url: &str, token: Option<String>) -> Driver {
    from(Client::new(url, token).expect("Failed to create Gitlab client"))
}

/// Returns a new Gitlab driver using the given client.
pub fn from(client: Client) -> Driver {
    Driver::Gitlab(GitlabDriver { client })
}

#[cfg(test)]
mod test {
    use crate::{http::Client, scm::driver::gitlab};

    use super::constants::GITLAB_ENDPOINT;

    #[test]
    fn create_gitlab_driver() {
        let _driver = gitlab::default();
    }

    #[test]
    fn create_gitlab_driver_from_client() {
        let _driver =
            gitlab::from(Client::new(GITLAB_ENDPOINT, None).expect("Failed to create Gitlab client"));
    }

    #[test]
    fn create_gitlab_selhost_driver() {
        let _driver = gitlab::new("https://gitlab.company.com", None);
    }
}
