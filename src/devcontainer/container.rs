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

use serde::{Deserialize, Serialize};

use super::types::StringOrArray;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContainer {
    /// The docker image that will be used to create the container.
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerfileContainer {
    /// The location of the Dockerfile that defines the contents of the container.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub dockerfile: String,
    /// The location of the context folder for building the Docker image.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Docker build-related options.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub options: Option<BuildOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOptions {
    /// Target stage in a multi-stage build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Build arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, String>>,
    /// The image to consider as a cache. Use an array to specify multiple images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_from: Option<StringOrArray>,
}

#[cfg(test)]
mod test {
    use crate::devcontainer::types::StringOrArray;

    #[test]
    fn test_deserialize_image_container() {
        let json = r#"
        {
            "image": "rust:1.55.0"
        }
        "#;

        let container: super::ImageContainer = serde_json::from_str(json).unwrap();
        assert_eq!(container.image, "rust:1.55.0");
    }

    #[test]
    fn test_deserialize_dockerfile_container() {
        let json = r#"
        {
            "dockerfile": "Dockerfile",
            "context": ".",
            "target": "dev",
            "args": {
                "RUST_VERSION": "1.55.0"
            },
            "cacheFrom": [
                "rust:1.55.0"
            ]
        }
        "#;

        let container: super::DockerfileContainer = serde_json::from_str(json).unwrap();
        assert_eq!(container.dockerfile, "Dockerfile");
        assert_eq!(container.context.unwrap(), ".");

        assert_eq!(container.options.is_some(), true);
        let options = container.options.as_ref().unwrap();
        assert_eq!(options.target.as_ref().unwrap(), "dev");
        assert_eq!(
            options.args.as_ref().unwrap().get("RUST_VERSION").unwrap(),
            "1.55.0"
        );
        assert_eq!(
            options.cache_from.as_ref().unwrap(),
            &StringOrArray::Array(vec!["rust:1.55.0".into()])
        );
    }
}
