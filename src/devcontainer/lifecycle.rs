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

/// When creating or working with a dev container, you may need different
/// commands to be run at different points in the container’s lifecycle.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleScripts {
    /// A command to run locally (i.e Your host machine, cloud VM) before anything else.
    /// This command is run before "onCreateCommand"". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub initialize_command: Option<LifecycleScript>,
    /// A command to run when creating the container.
    /// This command is run after "initializeCommand" and before "updateContentCommand".
    /// If this is a single string, it will be run in a shell. If this is an array of strings,
    /// it will be run as a single command without shell. If this is an object,
    /// each provided command will be run in parallel.
    pub on_create_command: Option<LifecycleScript>,
    /// A command to run when creating the container and rerun when the workspace content
    /// was updated while creating the container. This command is run after "onCreateCommand"
    /// and before "postCreateCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub update_content_command: Option<LifecycleScript>,
    /// A command to run after creating the container. This command is run after "updateContentCommand"
    /// and before "postStartCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub post_create_command: Option<LifecycleScript>,
    /// A command to run after starting the container. This command is run after "postCreateCommand"
    /// and before "postAttachCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.",
    pub post_start_command: Option<LifecycleScript>,
    /// A command to run when attaching to the container. This command is run after "postStartCommand".
    /// If this is a single string, it will be run in a shell. If this is an array of strings,
    /// it will be run as a single command without shell. If this is an object,
    /// each provided command will be run in parallel.
    pub post_attach_command: Option<LifecycleScript>,
    /// The user command to wait for before continuing execution in the background
    /// while the UI is starting up. The default is "updateContentCommand"
    pub wait_for: Option<WaitFor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LifecycleScript {
    String(String),
    Array(Vec<String>),
    Object(HashMap<String, String>),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Serialize, Deserialize)]
pub enum WaitFor {
    InitializeCommand,
    OnCreateCommand,
    UpdateContentCommand,
    PostCreateCommand,
    PostStartCommand,
}
