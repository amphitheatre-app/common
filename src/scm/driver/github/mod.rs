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

mod content;
mod driver;
mod git;
mod pr;
mod repo;
mod utils;

use self::driver::GithubDriver;
pub use self::pr::GithubFile;

/// Returnes a new GitHub API client.
pub fn new(url: &str, token: Option<String>) -> GithubDriver {
    GithubDriver {
        client: crate::client::Client::new(url, token),
    }
}

/// Returns a new GitHub API client using the default api.github.com address.
pub fn default() -> GithubDriver {
    new("https://api.github.com", None)
}

#[cfg(test)]
mod test {
    use crate::scm::driver::github;

    #[test]
    fn create_github_driver() {
        let _driver = github::default();
    }

    #[test]
    fn create_github_enterprise_driver() {
        let _driver = github::new("https://github.company.com/api/v3", None);
    }
}
