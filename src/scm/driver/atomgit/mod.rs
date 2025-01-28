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

pub mod constants;
pub mod content;
pub mod driver;
pub mod git;
pub mod pr;
pub mod repo;
mod utils;

use self::constants::ATOMGIT_ENDPOINT;
use self::pr::AtomGitFile;
use super::Driver;
use crate::http::Client;
use crate::scm::driver::atomgit::driver::AtomGitDriver;

/// Returns a new AtomGit driver using the default api.atomgit.com address.
#[inline]
pub fn default() -> Driver {
    from(Client::new(ATOMGIT_ENDPOINT, None).expect("Failed to create AtomGit client"))
}

/// Returns a new AtomGit driver.
#[inline]
pub fn new(url: &str, token: Option<String>) -> Driver {
    from(Client::new(url, token).expect("Failed to create AtomGit client"))
}

/// Returns a new AtomGit driver using the given client.
pub fn from(client: Client) -> Driver {
    Driver::AtomGit(AtomGitDriver { client })
}

#[cfg(test)]
mod test {
    use super::constants::ATOMGIT_ENDPOINT;
    use crate::{http::Client, scm::driver::atomgit};

    #[test]
    fn create_atomgit_driver() {
        let _driver = atomgit::default();
    }

    #[test]
    fn create_atomgit_driver_from_client() {
        let _driver =
            atomgit::from(Client::new(ATOMGIT_ENDPOINT, None).expect("Failed to create AtomGit client"));
    }

    #[test]
    fn create_atomgit_enterprise_driver() {
        let _driver = atomgit::new("https://atomgit.company.com", None);
    }
}
