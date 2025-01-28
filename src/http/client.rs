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

use reqwest::{
    header::{self, HeaderValue},
    ClientBuilder, RequestBuilder, StatusCode,
};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};
use url::Url;

use super::HTTPError;

/// Represents the Rust client for the API
///
/// The client is your entrypoint to the API. Using it you will be
/// able to call all the endpoints of the API and their respective functions.
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
    base_url: Url,
    client: reqwest::Client,
}

/// Defines the Endpoint trait for the different API endpoints
pub trait Endpoint {
    type Output: DeserializeOwned;
}

/// Represents the response from an API call
#[derive(Debug)]
pub struct Response<T> {
    /// The HTTP Status Code
    pub status: StatusCode,
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
    /// `base_url`: the base URL of the API
    /// `token`: the bearer authentication token
    pub fn new(base_url: &str, token: Option<String>) -> Result<Client, HTTPError> {
        let base_url = Url::parse(base_url).map_err(HTTPError::UrlParse)?;

        // Set the default headers for every request
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"))),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );

        if let Some(token) = token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).map_err(HTTPError::InvalidHeaderValue)?,
            );
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(HTTPError::ReqwestError)?;

        Ok(Client { base_url, client })
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
    pub async fn get<E: Endpoint>(
        &self,
        path: &str,
        options: Option<HashMap<String, String>>,
    ) -> Result<Response<E::Output>, HTTPError> {
        let mut request = self.client.get(self.url(path)?);
        if let Some(options) = options {
            for (key, value) in options {
                request = request.query(&[(key, value)]);
            }
        }
        self.execute::<E>(request).await
    }

    /// Sends a POST request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub async fn post<E: Endpoint>(
        &self,
        path: &str,
        data: &Value,
    ) -> Result<Response<<E as Endpoint>::Output>, HTTPError> {
        let request = self.client.post(self.url(path)?).json(data);
        self.execute::<E>(request).await
    }

    /// Sends a PUT request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub async fn put<E: Endpoint>(
        &self,
        path: &str,
        data: &Value,
    ) -> Result<Response<<E as Endpoint>::Output>, HTTPError> {
        let request = self.client.put(self.url(path)?).json(data);
        self.execute::<E>(request).await
    }

    /// Sends a PATCH request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub async fn patch<E: Endpoint>(
        &self,
        path: &str,
        data: &Value,
    ) -> Result<Response<<E as Endpoint>::Output>, HTTPError> {
        let request = self.client.patch(self.url(path)?).json(data);
        self.execute::<E>(request).await
    }

    /// Sends a DELETE request to the API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub async fn delete<E: Endpoint>(&self, path: &str) -> Result<Response<E::Output>, HTTPError> {
        self.execute::<E>(self.client.delete(self.url(path)?)).await
    }

    async fn execute<E: Endpoint>(&self, request: RequestBuilder) -> Result<Response<E::Output>, HTTPError> {
        let result = request.send().await;

        match result {
            Ok(response) => {
                let status = response.status();
                let json = response.json::<Value>().await.map_err(HTTPError::ReqwestError)?;
                let data = from_value(json.clone()).map_err(HTTPError::Deserialization)?;
                let body = Some(json);

                Ok(Response { status, data, body })
            }
            Err(err) => Err(HTTPError::ReqwestError(err)),
        }
    }

    #[inline]
    fn url(&self, path: &str) -> Result<Url, HTTPError> {
        self.base_url.join(path).map_err(HTTPError::UrlParse)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    const BASE_URL: &str = "https://cloud.amphitheatre.app/";

    #[test]
    fn creates_a_client() {
        let token = "some-auth-token";
        let client = Client::new(BASE_URL, Some(token.to_string())).unwrap();
        assert_eq!(client.base_url.as_str(), BASE_URL);
    }
}
