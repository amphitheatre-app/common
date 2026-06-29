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

use std::collections::HashMap;

use async_trait::async_trait;
use data_encoding::BASE64_MIME as BASE64;
use serde::{Deserialize, Serialize};

use super::constants::GOGS_PATH_CONTENTS;
use crate::http::{endpoint::Endpoint, Client};
use crate::scm::content::{Content, ContentService, File};
use crate::scm::errors::SCMError;

pub struct GogsContentService {
    pub client: Client,
}

#[async_trait]
impl ContentService for GogsContentService {
    /// Gets the contents of a file in a repository.
    ///
    /// Docs: https://gogs.io/docs/api
    /// Example: https://gogs.io/api/v1/repos/gogs/gogs/contents/README.md
    async fn find(&self, repo: &str, file: &str, reference: &str) -> Result<Content, SCMError> {
        let path = GOGS_PATH_CONTENTS.replace("{repo}", repo).replace("{file}", file);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<GogsContent>(&path, Some(options))
            .await
            .map_err(SCMError::ClientError)?;

        if let Some(content) = res.data {
            Ok(content.try_into().map_err(SCMError::DecodeError)?)
        } else {
            Err(SCMError::NotFound(path))
        }
    }

    /// Gets the file list of a directory in a repository.
    ///
    /// Docs: https://gogs.io/docs/api
    /// Example: https://gogs.io/api/v1/repos/gogs/gogs/contents
    async fn list(&self, repo: &str, path: &str, reference: &str) -> Result<Vec<File>, SCMError> {
        let path = GOGS_PATH_CONTENTS.replace("{repo}", repo).replace("{file}", path);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<Vec<GogsFile>>(&path, Some(options))
            .await
            .map_err(SCMError::ClientError)?;

        if let Some(list) = res.data {
            return Ok(list.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GogsContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl TryFrom<GogsContent> for Content {
    type Error = data_encoding::DecodeError;

    fn try_from(val: GogsContent) -> Result<Self, Self::Error> {
        Ok(Self {
            path: val.path,
            data: BASE64.decode(val.content.as_bytes())?,
            // the sha returned for gogs rest api is the commit sha, not the blob sha
            sha: val.sha.clone(),
            blob_id: val.sha,
        })
    }
}

impl Endpoint for GogsContent {
    type Output = GogsContent;
}

/// represents a file in a repository.
#[derive(Debug, Deserialize, Serialize)]
pub struct GogsFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl From<&GogsFile> for File {
    fn from(val: &GogsFile) -> Self {
        Self {
            name: val.name.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            blob_id: val.sha.clone(),
            kind: val.kind.clone(),
        }
    }
}

impl Endpoint for Vec<GogsFile> {
    type Output = Vec<GogsFile>;
}
