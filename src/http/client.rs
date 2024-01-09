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

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::{from_value, json, Value};
use ureq::Request;

use super::HTTPError;

const VERSION: &str = "0.1.0";
const DEFAULT_USER_AGENT: &str = "amp";

/// Represents the Rust client for the API
///
/// The client is your entrypoint to the API. Using it you will be
/// able to call all the enpoints of the API and their respective functions.
///
/// # Examples
///
/// ```no_run
/// use amp_common::http::Client;
///
/// let token = Some("AUTH_TOKEN".to_string());
/// let client = Client::new("https://cloud.amphitheatre.app", token);
/// ```
#[derive(Clone)]
pub struct Client {
    base_url: String,
    user_agent: String,
    auth_token: Option<String>,
    pub _agent: ureq::Agent,
}

/// Defines the Endpoint trait for the different API endpoints
pub trait Endpoint {
    type Output: DeserializeOwned;
}

/// Represents the response from an API call
#[derive(Debug)]
pub struct Response<T> {
    /// The HTTP Status Code
    pub status: u16,
    /// The object or a Vec<T> objects (the type `T` will depend on the endpoint).
    pub data: Option<T>,
    /// The body as a JSON `Value`
    pub body: Option<Value>,
}

impl Client {
    /// Helper function to create a new client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use amp_common::http::Client;
    /// let token = Some("AUTH_TOKEN".to_string());
    /// let client = Client::new("https://cloud.amphitheatre.app", token);
    /// ```
    ///
    /// # Arguments
    ///
    /// `token`: the bearer authentication token
    pub fn new(base_url: &str, token: Option<String>) -> Client {
        Client {
            base_url: String::from(base_url),
            user_agent: DEFAULT_USER_AGENT.to_owned() + "/" + VERSION,
            auth_token: token,
            _agent: ureq::Agent::new(),
        }
    }
}

impl Client {
    /// Sends a GET request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `options`: optionally a `RequestOptions` with things like pagination,
    /// filtering and sorting
    pub fn get<E: Endpoint>(
        &self,
        path: &str,
        options: Option<HashMap<String, String>>,
    ) -> Result<Response<E::Output>, HTTPError> {
        self.call::<E>(self.build_get_request(&path, options))
    }

    /// Sends a POST request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn post<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<Response<<E as Endpoint>::Output>, HTTPError> {
        self.call_with_payload::<E>(self.build_post_request(&path), data)
    }

    /// Sends a POST request to the API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn empty_post(&self, path: &str) -> Result<Response<()>, HTTPError> {
        self.call_empty(self.build_post_request(&path))
    }

    /// Sends a PUT request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn put<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<Response<<E as Endpoint>::Output>, HTTPError> {
        self.call_with_payload::<E>(self.build_put_request(&path), data)
    }

    /// Sends a PUT request to the API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn empty_put(&self, path: &str) -> Result<Response<()>, HTTPError> {
        self.call_empty(self.build_put_request(&path))
    }

    /// Sends a PATCH request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn patch<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<Response<<E as Endpoint>::Output>, HTTPError> {
        self.call_with_payload::<E>(self.build_patch_request(&path), data)
    }

    /// Sends a DELETE request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete(&self, path: &str) -> Result<Response<()>, HTTPError> {
        self.call_empty(self.build_delete_request(&path))
    }

    /// Sends a DELETE request to the API returning a response containing a `Response`
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete_with_response<E: Endpoint>(&self, path: &str) -> Result<Response<E::Output>, HTTPError> {
        self.call::<E>(self.build_delete_request(&path))
    }

    fn call_with_payload<E: Endpoint>(
        &self,
        request: Request,
        data: Value,
    ) -> Result<Response<E::Output>, HTTPError> {
        self.process_response::<E>(request.send_json(data))
    }

    fn call<E: Endpoint>(&self, request: Request) -> Result<Response<E::Output>, HTTPError> {
        self.process_response::<E>(request.call())
    }

    fn process_response<E: Endpoint>(
        &self,
        result: Result<ureq::Response, ureq::Error>,
    ) -> Result<Response<E::Output>, HTTPError> {
        match result {
            Ok(response) => Self::build_response::<E>(response),
            Err(ureq::Error::Status(code, response)) => Err(HTTPError::parse_response(code, response)),
            Err(ureq::Error::Transport(transport)) => Err(HTTPError::parse_transport(transport)),
        }
    }

    fn call_empty(&self, request: Request) -> Result<Response<()>, HTTPError> {
        match request.call() {
            Ok(response) => Self::build_empty_response(response),
            Err(ureq::Error::Status(code, response)) => Err(HTTPError::parse_response(code, response)),
            Err(ureq::Error::Transport(transport)) => Err(HTTPError::parse_transport(transport)),
        }
    }

    fn build_response<E: Endpoint>(resp: ureq::Response) -> Result<Response<E::Output>, HTTPError> {
        let status = resp.status();

        let json = resp
            .into_json::<Value>()
            .map_err(|e| HTTPError::Deserialization(e.to_string()))?;
        let data = from_value(json!(json)).map_err(|e| HTTPError::Deserialization(e.to_string()))?;
        let body = from_value(json!(json)).map_err(|e| HTTPError::Deserialization(e.to_string()))?;

        Ok(Response { status, data, body })
    }

    fn build_empty_response(res: ureq::Response) -> Result<Response<()>, HTTPError> {
        Ok(Response {
            status: res.status(),
            data: None,
            body: None,
        })
    }

    fn build_get_request(&self, path: &&str, options: Option<HashMap<String, String>>) -> Request {
        let mut request = self
            ._agent
            .get(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");

        if let Some(options) = options {
            for (key, value) in options {
                request = request.query(&key, &value);
            }
        }

        self.add_headers_to_request(request)
    }

    pub fn build_post_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .post(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    pub fn build_put_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .put(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    pub fn build_patch_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .request("PATCH", &self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn build_delete_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .delete(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn add_headers_to_request(&self, request: Request) -> Request {
        let mut request = request;
        if let Some(token) = &self.auth_token {
            let auth_token = &format!("Bearer {}", token);
            request = request.set("Authorization", auth_token.as_str());
        }

        request
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, DEFAULT_USER_AGENT, VERSION};
    const BASE_URL: &str = "https://cloud.amphitheatre.app";

    #[test]
    fn creates_a_client() {
        let token = "some-auth-token";
        let client = Client::new(BASE_URL, Some(token.to_string()));

        assert_eq!(client.base_url, BASE_URL);
        assert_eq!(client.user_agent, DEFAULT_USER_AGENT.to_owned() + "/" + VERSION);
        assert_eq!(client.auth_token, Some(token.to_string()));
    }
}
