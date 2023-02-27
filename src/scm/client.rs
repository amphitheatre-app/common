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
}

#[cfg(test)]
mod test {
    use crate::scm::client::Client;
    use crate::scm::driver::github;
    use crate::scm::git::GitService;

    #[test]
    fn call_get_service() {
        let client = Client::new(github::default());
        let commit = client.git().find_commit("octocat/Hello-World", "master");

        assert!(commit.is_ok());
    }
}
