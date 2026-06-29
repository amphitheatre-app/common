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

pub const GOGS_ENDPOINT: &str = "https://gogs.io";

// REST API paths (see https://gogs.io/docs/api)
pub const GOGS_PATH_CONTENTS: &str = "/api/v1/repos/{repo}/contents/{file}";
pub const GOGS_PATH_BRANCHES: &str = "/api/v1/repos/{repo}/branches";
pub const GOGS_PATH_TAGS: &str = "/api/v1/repos/{repo}/tags";
pub const GOGS_PATH_COMMITS: &str = "/api/v1/repos/{repo}/git/commits/{reference}";
pub const GOGS_PATH_GIT_TREES: &str = "/api/v1/repos/{repo}/git/trees/{tree_sha}";
pub const GOGS_PATH_REPOS: &str = "/api/v1/repos/{repo}";
