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

use self::driver::GitlabDriver;
use super::Driver;
use crate::client::Client;

pub mod content;
pub mod driver;
pub mod git;
pub mod repo;
mod utils;

/// Returns a new Gitlab API client.
pub fn new(url: &str, token: Option<String>) -> Driver {
    Driver::Gitlab(GitlabDriver {
        client: Client::new(url, token),
    })
}

/// Returns a new Gitlab API client using the default gitlab.com address.
pub fn default() -> Driver {
    new("https://gitlab.com", None)
}

#[cfg(test)]
mod test {
    use crate::scm::driver::gitlab;

    #[test]
    fn create_gitlab_driver() {
        let _driver = gitlab::default();
    }

    #[test]
    fn create_gitlab_selhost_driver() {
        let _driver = gitlab::new("https://gitlab.company.com", None);
    }
}
