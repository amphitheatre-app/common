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

use self::constants::GOGS_ENDPOINT;
use self::pr::GogsFile;
use super::Driver;
use crate::http::Client;
use crate::scm::driver::gogs::driver::GogsDriver;
use crate::scm::errors::SCMError;

/// Returns a new Gogs driver using the default gogs.io address.
#[inline]
pub fn default() -> Result<Driver, SCMError> {
    from(Client::new(GOGS_ENDPOINT, None).map_err(SCMError::ClientError)?)
}

/// Returns a new Gogs driver.
#[inline]
pub fn new(url: &str, token: Option<String>) -> Result<Driver, SCMError> {
    from(Client::new(url, token).map_err(SCMError::ClientError)?)
}

/// Returns a new Gogs driver using the given client.
pub fn from(client: Client) -> Result<Driver, SCMError> {
    Ok(Driver::Gogs(GogsDriver { client }))
}

#[cfg(test)]
mod test {
    use super::constants::GOGS_ENDPOINT;
    use crate::{http::Client, scm::driver::gogs};

    #[test]
    fn create_gogs_driver() {
        let _driver = gogs::default();
    }

    #[test]
    fn create_gogs_driver_from_client() {
        let _driver = gogs::from(Client::new(GOGS_ENDPOINT, None).unwrap());
    }

    #[test]
    fn create_gogs_enterprise_driver() {
        let _driver = gogs::new("https://gogs.company.com", None);
    }
}
