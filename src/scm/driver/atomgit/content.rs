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

use data_encoding::BASE64_MIME as BASE64;
use serde::{Deserialize, Serialize};

use super::constants::ATOMGIT_PATH_CONTENTS;
use crate::http::{Client, Endpoint};
use crate::scm::content::{Content, ContentService, File};

pub struct AtomgitContentService {
    pub client: Client,
}

impl ContentService for AtomgitContentService {
    /// Gets the contents of a file in a repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repo-conent/
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/contents/
    fn find(&self, repo: &str, file: &str, reference: &str) -> anyhow::Result<Content> {
        let path = ATOMGIT_PATH_CONTENTS
            .replace("{repo}", repo)
            .replace("{file}", file);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self.client.get::<GithubContentEndpoint>(&path, Some(options))?;

        if let Some(content) = res.data {
            Ok(content.try_into()?)
        } else {
            Err(anyhow::anyhow!("Not found: {}", path))
        }
    }

    /// Gets the file list of a directory in a repository.
    ///
    /// Docs: https://docs.atomgit.com/en/openAPI/api_versioned/get-repo-conent/
    /// Example: https://api.atomgit.com/repos/jia-hao-li/atomgit_evaluation/contents/jia-hao-li
    fn list(&self, repo: &str, path: &str, reference: &str) -> anyhow::Result<Vec<File>> {
        let path = ATOMGIT_PATH_CONTENTS
            .replace("{repo}", repo)
            .replace("{file}", path);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self.client.get::<GithubFileEndpoint>(&path, Some(options))?;

        if let Some(list) = res.data {
            return Ok(list.iter().map(|v| v.into()).collect());
        }

        Ok(vec![])
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl TryFrom<AtomgitContent> for Content {
    type Error = data_encoding::DecodeError;

    fn try_from(val: AtomgitContent) -> Result<Self, Self::Error> {
        Ok(Self {
            path: val.path,
            data: BASE64.decode(val.content.as_bytes())?,
            // the sha returned for github rest api is the blob sha, not the commit sha
            sha: val.sha.clone(),
            blob_id: val.sha,
        })
    }
}

struct GithubContentEndpoint;

impl Endpoint for GithubContentEndpoint {
    type Output = AtomgitContent;
}

/// represents a file in a repository.
#[derive(Debug, Deserialize, Serialize)]
pub struct AtomgitFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    #[serde(rename = "type")]
    pub kind: String,
}

impl From<&AtomgitFile> for File {
    fn from(val: &AtomgitFile) -> Self {
        Self {
            name: val.name.clone(),
            path: val.path.clone(),
            sha: val.sha.clone(),
            blob_id: val.sha.clone(),
            kind: val.kind.clone(),
        }
    }
}

struct GithubFileEndpoint;

impl Endpoint for GithubFileEndpoint {
    type Output = Vec<AtomgitFile>;
}
