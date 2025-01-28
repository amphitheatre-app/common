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

use super::constants::GITHUB_PATH_CONTENTS;
use crate::http::{endpoint::Endpoint, Client};
use crate::scm::content::{Content, ContentService, File};
use crate::scm::errors::SCMError;

pub struct GithubContentService {
    pub client: Client,
}

#[async_trait]
impl ContentService for GithubContentService {
    /// Gets the contents of a file in a repository.
    ///
    /// Docs: https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-repository-content
    /// Example: https://api.github.com/repos/octocat/Hello-World/contents/README
    async fn find(&self, repo: &str, file: &str, reference: &str) -> Result<Content, SCMError> {
        let path = GITHUB_PATH_CONTENTS
            .replace("{repo}", repo)
            .replace("{file}", file);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<GithubContent>(&path, Some(options))
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
    /// Docs: https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-repository-content
    /// Example: https://api.github.com/repos/octocat/Hello-World/contents/
    async fn list(&self, repo: &str, path: &str, reference: &str) -> Result<Vec<File>, SCMError> {
        let path = GITHUB_PATH_CONTENTS
            .replace("{repo}", repo)
            .replace("{file}", path);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<Vec<GithubFile>>(&path, Some(options))
            .await
            .map_err(SCMError::ClientError)?;

        if let Some(list) = res.data {
            return Ok(list.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl TryFrom<GithubContent> for Content {
    type Error = data_encoding::DecodeError;

    fn try_from(val: GithubContent) -> Result<Self, Self::Error> {
        Ok(Self {
            path: val.path,
            data: BASE64.decode(val.content.as_bytes())?,
            // the sha returned for github rest api is the blob sha, not the commit sha
            sha: val.sha.clone(),
            blob_id: val.sha,
        })
    }
}

impl Endpoint for GithubContent {
    type Output = GithubContent;
}

/// represents a file in a repository.
#[derive(Debug, Deserialize, Serialize)]
pub struct GithubFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl From<&GithubFile> for File {
    fn from(val: &GithubFile) -> Self {
        Self {
            name: val.name.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            blob_id: val.sha.clone(),
            kind: val.kind.clone(),
        }
    }
}

impl Endpoint for Vec<GithubFile> {
    type Output = Vec<GithubFile>;
}
