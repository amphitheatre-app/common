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

pub mod client;

use self::client::GithubClient;
use crate::scm::client::Client;

/// Returnes a new GitHub API client.
pub fn new(_uri: &str) -> anyhow::Result<impl Client> {
    Ok(GithubClient {})
}

/// Returns a new GitHub API client using the default api.github.com address.
pub fn default() -> anyhow::Result<impl Client> {
    new("https://api.github.com")
}

#[cfg(test)]
mod test {
    use crate::scm::driver::github;

    #[test]
    fn create_github_client() {
        let _client = github::default();
    }

    #[test]
    fn create_github_enterprise_client() {
        let _client = github::new("https://github.company.com/api/v3");
    }
}
