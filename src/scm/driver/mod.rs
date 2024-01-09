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

pub mod github;
pub mod gitlab;

use super::errors::SCMError;
use crate::config::RepositoryCredential;
use crate::scm::content::ContentService;
use crate::scm::git::GitService;
use crate::scm::repo::RepositoryService;
use crate::utils::http::host;

/// Driver is a enum that represents the SCM driver.
pub enum Driver {
    Github(github::driver::GithubDriver),
    Gitlab(gitlab::driver::GitlabDriver),
}

/// Defines the methods that a SCM driver must implement.
pub trait DriverTrait {
    fn contents(&self) -> Box<dyn ContentService>;
    fn git(&self) -> Box<dyn GitService>;
    fn repositories(&self) -> Box<dyn RepositoryService>;
}

impl DriverTrait for Driver {
    fn contents(&self) -> Box<dyn ContentService> {
        match self {
            Driver::Github(driver) => driver.contents(),
            Driver::Gitlab(driver) => driver.contents(),
        }
    }

    fn git(&self) -> Box<dyn GitService> {
        match self {
            Driver::Github(driver) => driver.git(),
            Driver::Gitlab(driver) => driver.git(),
        }
    }

    fn repositories(&self) -> Box<dyn RepositoryService> {
        match self {
            Driver::Github(driver) => driver.repositories(),
            Driver::Gitlab(driver) => driver.repositories(),
        }
    }
}

impl TryFrom<&RepositoryCredential> for Driver {
    type Error = SCMError;

    fn try_from(credential: &RepositoryCredential) -> Result<Self, Self::Error> {
        match credential.driver.as_str() {
            "github" => Ok(github::new(&credential.server, credential.token.clone())),
            "gitlab" => Ok(gitlab::new(&credential.server, credential.token.clone())),
            _ => Err(SCMError::UnknownDriver(credential.driver.to_string())),
        }
    }
}

impl TryFrom<&str> for Driver {
    type Error = SCMError;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        let server = host(url).ok_or_else(|| SCMError::InvalidRepoAddress(url.to_string()))?;
        match server.as_str() {
            "github.com" => Ok(github::default()),
            "gitlab.com" => Ok(gitlab::default()),
            _ => Err(SCMError::UnknownDriver(url.to_string())),
        }
    }
}
