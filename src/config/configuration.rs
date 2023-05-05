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

use std::path::{Path, PathBuf};

use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use super::ContextConfiguration;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Configuration {
    /// the configuration of the context
    pub context: Option<ContextConfiguration>,
}

impl Configuration {
    /// get the default configuration path
    pub fn path() -> Result<PathBuf, confy::ConfyError> {
        let project = ProjectDirs::from("", "", "Amphitheatre").ok_or_else(|| {
            confy::ConfyError::BadConfigDirectory("could not determine home directory path".to_string())
        })?;

        let config_dir_str = get_configuration_directory_str(&project)?;

        let path = [config_dir_str, "config.toml"].iter().collect();

        Ok(path)
    }

    /// load configuration from the specified path
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Configuration> {
        confy::load_path(path).with_context(|| "unable to load configuration")
    }

    /// save configuration to the specified path
    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        confy::store_path(path, self).with_context(|| "unable to save configuration")
    }
}

fn get_configuration_directory_str(project: &ProjectDirs) -> Result<&str, confy::ConfyError> {
    let path = project.config_dir();
    path.to_str()
        .ok_or_else(|| confy::ConfyError::BadConfigDirectory(format!("{:?} is not valid Unicode", path)))
}
