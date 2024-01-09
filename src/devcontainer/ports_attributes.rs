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

use serde::{Deserialize, Serialize};

/// The PortAttributes properties allow you to map default port options
/// for one or more manually or automatically forwarded ports.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortAttributes {
    /// Defines the action that occurs when the port is discovered for automatic forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_auto_forward: Option<OnAutoForward>,
    /// Automatically prompt for elevation (if needed) when this port is forwarded.
    /// Elevate is required if the local port is a privileged port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevate_if_needed: Option<bool>,
    /// Label that will be shown in the UI for this port.
    // TODO: default: Application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// When true, a modal dialog will show if the chosen local port isn't used for forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_local_port: Option<bool>,
    /// The protocol to use when forwarding this port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}

impl Default for PortAttributes {
    fn default() -> Self {
        Self {
            on_auto_forward: Some(OnAutoForward::Notify),
            elevate_if_needed: Some(false),
            label: Some("Application".to_string()),
            require_local_port: Some(false),
            protocol: None,
        }
    }
}

/// Defines the action that occurs when the port is discovered for automatic forwarding
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OnAutoForward {
    /// Shows a notification when a port is automatically forwarded.
    // TODO: default
    Notify,
    /// Opens the browser when the port is automatically forwarded.
    /// Depending on your settings, this could open an embedded browser.
    OpenBrowser,
    /// Opens the browser when the port is automatically forwarded,
    /// but only the first time the port is forward during a session.
    /// Depending on your settings, this could open an embedded browser.
    OpenBrowserOnce,
    /// Opens a preview in the same window when the port is automatically forwarded.
    OpenPreview,
    /// Shows no notification and takes no action when this port is automatically forwarded.
    Silent,
    /// This port will not be automatically forwarded.
    Ignore,
}

/// The Protocol to use when forwarding a port.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    Http,
    Https,
}
