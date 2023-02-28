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

/// Returns ref without the path prefix.
pub fn trim_ref(reference: &str) -> String {
    let mut reference = String::from(reference);
    reference = reference.trim_start_matches("refs/heads/").to_string();
    reference = reference.trim_start_matches("refs/tags/").to_string();
    reference
}

/// Returns name expanded to the fully qualified reference path (e.g refs/heads/master).
pub fn expand_ref(name: &str, prefix: &str) -> String {
    let mut prefix = String::from(prefix);
    prefix = prefix.trim_end_matches('/').to_string();

    if name.starts_with("refs/") {
        return name.to_string();
    }

    format!("{}/{}", prefix, name)
}
