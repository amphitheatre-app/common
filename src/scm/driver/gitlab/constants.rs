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

pub const GITLAB_ENDPOINT: &str = "https://gitlab.com";

// REST API paths
pub const GITLAB_PATH_CONTENTS: &str = "/api/v4/projects/{repo}/repository/files/{file}";
pub const GITLAB_PATH_BRANCHES: &str = "/api/v4/projects/{repo}/repository/branches";
pub const GITLAB_PATH_TAGS: &str = "/api/v4/projects/{repo}/repository/tags";
pub const GITLAB_PATH_COMMITS: &str = "/api/v4/projects/{repo}/repository/commits/{reference}";
pub const GITLAB_PATH_REPOS: &str = "/api/v4/projects/{repo}";
