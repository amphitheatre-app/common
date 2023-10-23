use thiserror::Error;

use crate::http::HTTPError;

#[derive(Debug, Error)]
pub enum SCMError {
    #[error("ClientError: {0}")]
    ClientError(#[source] HTTPError),

    #[error("InvalidRepoAddress: {0}")]
    InvalidRepoAddress(String),

    #[error("UnknownDriver: {0}")]
    UnknownDriver(String),

    #[error("InvalidHostname")]
    InvalidHostname,
}
