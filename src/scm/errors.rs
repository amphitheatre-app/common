use thiserror::Error;

use crate::client::ClientError;

#[derive(Debug, Error)]
pub enum SCMError {
    #[error("ClientError: {0}")]
    ClientError(#[source] ClientError),

    #[error("InvalidRepoAddress: {0}")]
    InvalidRepoAddress(String),

    #[error("UnkownDriver: {0}")]
    UnkownDriver(String),

    #[error("InvalidHostname")]
    InvalidHostname,
}
