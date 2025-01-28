mod client;
pub mod endpoint;
mod errors;

use bytes::Bytes;
use reqwest::StatusCode;

// Re-export the client and errors
pub use self::client::*;
pub use self::errors::*;

/// Represents the response from an API call
#[derive(Debug)]
pub struct Response<T> {
    /// The HTTP Status Code
    pub status: StatusCode,
    /// The object or a Vec<T> objects (the type `T` will depend on the endpoint).
    pub data: Option<T>,
    /// The body's raw bytes
    pub body: Option<Bytes>,
}
