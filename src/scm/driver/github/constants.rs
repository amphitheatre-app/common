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

pub const GITHUB_ENDPOINT: &str = "https://api.github.com";

// REST API paths
pub const GITHUB_PATH_CONTENTS: &str = "/repos/{repo}/contents/{file}";
pub const GITHUB_PATH_BRANCHES: &str = "/repos/{repo}/branches";
pub const GITHUB_PATH_TAGS: &str = "/repos/{repo}/tags";
pub const GITHUB_PATH_COMMITS: &str = "/repos/{repo}/commits/{reference}";
pub const GITHUB_PATH_REPOS: &str = "/repos/{repo}";
pub const GITHUB_PATH_GIT_TREES: &str = "/repos/{repo}/git/trees/{tree_sha}";
