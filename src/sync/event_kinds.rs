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

use notify::event::{CreateKind, DataChange, MetadataKind, ModifyKind, RemoveKind};
use notify::EventKind::{self, Create, Modify, Remove};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// EventKinds
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum EventKinds {
    /// for initial file synchronization, full override.
    Overwrite,
    /// Include CreateKind::File and CreateKind::Folder
    Create,
    /// Modify event kind just only for file content change.
    Modify,
    /// Modify the file or folder name.
    Rename,
    /// Include RemoveKind::File and RemoveKind::Folder,
    /// because FSEvent has bug, so ignore this event now.
    /// @TODO: fix EventKinds::Remove ignore issue.
    Remove,
    /// Other event kinds
    Other,
}

/// Convert EventKinds to EventKind
impl From<EventKind> for EventKinds {
    fn from(kind: EventKind) -> Self {
        match kind {
            Create(CreateKind::File) | Create(CreateKind::Folder) => Self::Create,
            Modify(ModifyKind::Data(DataChange::Content)) => Self::Modify,
            // Special case for metadata change on macOS when duplicate file or folder.
            Modify(ModifyKind::Metadata(MetadataKind::Any)) => Self::Modify,
            Modify(ModifyKind::Name(_)) => Self::Rename,
            Remove(RemoveKind::File) | Remove(RemoveKind::Folder) => Self::Remove,
            _ => Self::Other,
        }
    }
}

/// Convert String to EventKinds
impl From<String> for EventKinds {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Overwrite" => Self::Overwrite,
            "Create" => Self::Create,
            "Modify" => Self::Modify,
            "Rename" => Self::Rename,
            "Remove" => Self::Remove,
            _ => Self::Other,
        }
    }
}
