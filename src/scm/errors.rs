use thiserror::Error;

use crate::http::HTTPError;

#[derive(Debug, Error)]
pub enum SCMError {
    #[error("Client Error: {0}")]
    ClientError(#[source] HTTPError),

    #[error("Invalid Repo Address: {0}")]
    InvalidRepoAddress(String),

    #[error("Unknown Driver: {0}")]
    UnknownDriver(String),

    #[error("Invalid Hostname")]
    InvalidHostname,

    #[error("Decode Error: {0}")]
    DecodeError(#[source] data_encoding::DecodeError),

    #[error("Not Found: {0}")]
    NotFound(String),
}
