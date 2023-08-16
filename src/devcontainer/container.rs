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

use super::types::StringOrArray;

pub struct ImageContainer {
    /// The docker image that will be used to create the container.
    pub image: String,
}

pub struct DockerfileContainer {
    /// The location of the Dockerfile that defines the contents of the container.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub dockerfile: String,
    /// The location of the context folder for building the Docker image.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub context: Option<String>,
    /// Docker build-related options.
    pub options: Option<BuildOptions>,
}

pub struct BuildOptions {
    /// Target stage in a multi-stage build.
    pub target: Option<String>,
    /// Build arguments.
    pub args: Option<HashMap<String, String>>,
    /// The image to consider as a cache. Use an array to specify multiple images.
    pub cache_from: Option<StringOrArray>,
}
