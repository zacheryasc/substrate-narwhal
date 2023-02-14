use thiserror::Error;

use crate::types::Round;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Certificate error: {0}")]
    Certificate(#[from] CertificateError),

    #[error("Failed to send message: {0}")]
    Sending(String),
}

#[derive(Debug, Error)]
pub enum CertificateError {
    #[error("Certificate is dated past garbage collection round. Certificate round: ({certificate_round}), GC round: {gc_round}")]
    Stale {
        certificate_round: Round,
        gc_round: Round,
    },
}

macro_rules! from_error {
    ($error:expr) => {
        |e| $error(e.to_string())
    };
}
pub(crate) use from_error;
