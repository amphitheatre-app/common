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

use crate::client::Client;
use crate::scm::git::{Commit, GitService, Reference};

pub struct GitHubGitService {
    pub client: Client,
}

impl GitService for GitHubGitService {
    fn list_branches(&self, _repo: &str) -> anyhow::Result<Vec<Reference>> {
        Ok(vec![])
    }

    fn list_tags(&self, _repo: &str) -> anyhow::Result<Vec<Reference>> {
        Ok(vec![])
    }

    fn find_commit(&self, _repo: &str, _reference: &str) -> anyhow::Result<Commit> {
        Ok(Commit::default())
    }
}

#[cfg(test)]
mod test {
    use super::GitHubGitService;
    use crate::client::Client;
    use crate::scm::git::{Commit, GitService};

    #[test]
    fn test_list_branches() {
        let service = GitHubGitService {
            client: Client::new("https://api.github.com", ""),
        };
        let result = service.list_branches("octocat/Hello-World");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }

    #[test]
    fn test_list_tags() {
        let service = GitHubGitService {
            client: Client::new("https://api.github.com", ""),
        };
        let result = service.list_tags("octocat/Hello-World");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }

    #[test]
    fn test_find_commit() {
        let service = GitHubGitService {
            client: Client::new("https://api.github.com", ""),
        };
        let result = service.find_commit("octocat/Hello-World", "master");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Commit::default());
    }
}
