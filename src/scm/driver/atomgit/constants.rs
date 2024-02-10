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

pub const ATOMGIT_ENDPOINT: &str = "https://api.atomgit.com";

// REST API paths
pub const ATOMGIT_PATH_CONTENTS: &str = "/repos/{repo}/contents/{file}";
pub const ATOMGIT_PATH_BRANCHES: &str = "/repos/{repo}/branches";
pub const ATOMGIT_PATH_TAGS: &str = "/repos/{repo}/tags";
pub const ATOMGIT_PATH_COMMITS: &str = "/repos/{repo}/commits/{reference}";
pub const ATOMGIT_PATH_REPOS: &str = "/repos/{repo}";
pub const ATOMGIT_PATH_GIT_TREES: &str = "/repos/{repo}/trees/{tree_sha}";
