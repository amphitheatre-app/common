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

use crate::client::{Client, Endpoint};
use crate::scm::content::{Content, ContentService};
pub struct GithubContentService {
    pub client: Client,
}

impl ContentService for GithubContentService {
    /// Returns a repository by name.
    fn find(&self, repo: &str, path: &str, reference: &str) -> anyhow::Result<Content> {
        let path = format!("/repos/{}/contents/{}", repo, path);
        let options = HashMap::from([("ref".to_string(), reference.to_string())]);
        let res = self.client.get::<GithubContentEndpoint>(&path, Some(options))?;

        if let Some(content) = res.data {
            Ok(content.try_into()?)
        } else {
            Err(anyhow::anyhow!("Not found: {}", path))
        }
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

struct GithubContentEndpoint;

impl Endpoint for GithubContentEndpoint {
    type Output = GithubContent;
}

#[cfg(test)]
mod test {
    use crate::client::Client;
    use crate::scm::content::ContentService;
    use crate::scm::driver::github::content::GithubContentService;

    #[test]
    fn test_find() {
        let service = GithubContentService {
            client: Client::new("https://api.github.com", None),
        };
        let result = service.find("octocat/Hello-World", "README", "master");
        println!("{:#?}", result);
        assert!(result.is_ok());

        let repo = result.unwrap();
        assert_eq!(repo.sha, "980a0d5f19a64b4b30a87d4206aade58726b60e3".to_string());
        assert_eq!(repo.data, "Hello World!\n".as_bytes());
    }
}
