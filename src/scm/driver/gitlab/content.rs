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

use std::collections::HashMap;

use data_encoding::BASE64_MIME as BASE64;
use serde::{Deserialize, Serialize};

use super::utils::{encode, encode_path};
use crate::http::{Client, Endpoint};
use crate::scm::content::{Content, ContentService};

pub struct GitlabContentService {
    pub client: Client,
}

impl ContentService for GitlabContentService {
    fn find(&self, repo: &str, path: &str, reference: &str) -> anyhow::Result<Content> {
        let path = format!(
            "/api/v4/projects/{}/repository/files/{}",
            encode(repo),
            encode_path(path)
        );
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self.client.get::<GitlabContentEndpoint>(&path, Some(options))?;

        if let Some(content) = res.data {
            Ok(content.try_into()?)
        } else {
            Err(anyhow::anyhow!("Not found: {}", path))
        }
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

#[cfg(test)]
mod test {
    use super::GitlabContentService;
    use crate::http::Client;
    use crate::scm::content::ContentService;

    #[test]
    fn test_find() {
        let service = GitlabContentService {
            client: Client::new("https://gitlab.com", None),
        };

        let result = service.find("gitlab-org/gitlab-test", "VERSION", "master");
        println!("{:?}", result);
        assert!(result.is_ok());

        let repo = result.unwrap();
        assert_eq!(repo.sha, "913c66a37b4a45b9769037c55c2d238bd0942d2e".to_string());
        assert_eq!(repo.data, "6.7.0.pre\n".as_bytes());
    }
}
