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

use super::constants::GITLAB_PATH_CONTENTS;
use super::utils::{encode, encode_path};
use crate::http::{Client, Endpoint};
use crate::scm::content::{Content, ContentService};

pub struct GitlabContentService {
    pub client: Client,
}

#[async_trait]
impl ContentService for GitlabContentService {
    /// Get file from repository.
    ///
    /// Docs: https://docs.gitlab.com/ee/api/repository_files.html#get-file-from-repository
    /// Example: https://gitlab.com/api/v4/projects/gitlab-org%2Fgitlab-test/repository/files/VERSION?ref=master
    async fn find(&self, repo: &str, file: &str, reference: &str) -> anyhow::Result<Content> {
        let path = GITLAB_PATH_CONTENTS
            .replace("{repo}", &encode(repo))
            .replace("{file}", &encode_path(file));
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self
            .client
            .get::<GitlabContentEndpoint>(&path, Some(options))
            .await?;

        if let Some(content) = res.data {
            Ok(content.try_into()?)
        } else {
            Err(anyhow::anyhow!("Not found: {}", path))
        }
    }

    /// @TODO: Get file list from repository.
    async fn list(
        &self,
        _repo: &str,
        _path: &str,
        _reference: &str,
    ) -> anyhow::Result<Vec<crate::scm::content::File>> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitlabContent {
    pub file_name: String,
    pub file_path: String,
    pub size: u64,
    pub encoding: String,
    pub content: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub blob_id: String,
    pub commit_id: String,
    pub last_commit_id: String,
}

impl TryFrom<GitlabContent> for Content {
    type Error = data_encoding::DecodeError;

    fn try_from(val: GitlabContent) -> Result<Self, Self::Error> {
        Ok(Self {
            path: val.file_path,
            data: BASE64.decode(val.content.as_bytes())?,
            sha: val.last_commit_id,
            blob_id: val.blob_id,
        })
    }
}

struct GitlabContentEndpoint;

impl Endpoint for GitlabContentEndpoint {
    type Output = GitlabContent;
}
