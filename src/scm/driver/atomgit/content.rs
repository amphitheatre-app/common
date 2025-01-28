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

use super::constants::ATOMGIT_PATH_CONTENTS;
use crate::http::{endpoint::Endpoint, Client};
use crate::scm::content::{Content, ContentService, File};
use crate::scm::errors::SCMError;

pub struct AtomGitContentService {
    pub client: Client,
}

#[async_trait]
impl ContentService for AtomGitContentService {
    /// Gets the contents of a file in a repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repo-conent/
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/contents/jia-hao-li
    async fn find(&self, repo: &str, file: &str, reference: &str) -> Result<Content, SCMError> {
        let path = ATOMGIT_PATH_CONTENTS
            .replace("{repo}", repo)
            .replace("{file}", file);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<AtomGitContent>(&path, Some(options))
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
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repo-conent/
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/contents/jia-hao-li
    async fn list(&self, repo: &str, path: &str, reference: &str) -> Result<Vec<File>, SCMError> {
        let path = ATOMGIT_PATH_CONTENTS
            .replace("{repo}", repo)
            .replace("{file}", path);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<Vec<AtomGitFile>>(&path, Some(options))
            .await
            .map_err(SCMError::ClientError)?;

        if let Some(list) = res.data {
            return Ok(list.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl TryFrom<AtomGitContent> for Content {
    type Error = data_encoding::DecodeError;

    fn try_from(val: AtomGitContent) -> Result<Self, Self::Error> {
        Ok(Self {
            path: val.path,
            data: BASE64.decode(val.content.as_bytes())?,
            // the sha returned for atomgit rest api is the blob sha, not the commit sha
            sha: val.sha.clone(),
            blob_id: val.sha,
        })
    }
}

impl Endpoint for AtomGitContent {
    type Output = AtomGitContent;
}

/// represents a file in a repository.
#[derive(Debug, Deserialize, Serialize)]
pub struct AtomGitFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl From<&AtomGitFile> for File {
    fn from(val: &AtomGitFile) -> Self {
        Self {
            name: val.name.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            blob_id: val.sha.clone(),
            kind: val.kind.clone(),
        }
    }
}

impl Endpoint for Vec<AtomGitFile> {
    type Output = Vec<AtomGitFile>;
}
