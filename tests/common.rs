// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Heavily inspired by https://github.com/dnsimple/dnsimple-rust

use std::fs;

use amp_common::http::Client;
use mockito::{Server, ServerGuard};

/// Creates a mock server and a client (changing the url of the client
/// to that of the mock server to capture the requests).
///
/// It builds a response struct for the mock server using the fixture.
///
/// # Arguments
///
/// `method`: the HTTP method we are going to use (GET, POST, DELETE, ...)
/// `path`: the path in the server (i.e. `/me`)
/// `fixture`: the path to the fixture inside the `api` directory
pub async fn mock(method: &str, path: &str, fixture: &str) -> (Client, ServerGuard) {
    let file = format!("./tests/fixtures/{}.http", fixture);
    let content =
        fs::read_to_string(file).unwrap_or_else(|_| panic!("Couldn't read the fixture file: {}", fixture));

    // Split the raw response into headers and body parts
    let parts: Vec<&str> = content.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        panic!("Invalid fixture");
    }

    // Parse the status line
    let status_line = parts[0].lines().next().unwrap();
    let mut status_parts = status_line.split_whitespace();
    let _ = status_parts.next().expect("Invalid version");
    let status_code = status_parts.next().expect("Invalid status code");

    let mut server = Server::new_async().await;
    server
        .mock(method, path)
        .match_query(mockito::Matcher::Any)
        .with_status(status_code.parse().unwrap())
        .with_body(parts[1])
        .create_async()
        .await;
    let client = Client::new(&server.url(), None).expect("Failed to create client");

    (client, server)
}
