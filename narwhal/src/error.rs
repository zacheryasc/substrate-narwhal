use thiserror::Error;

use crate::types::Round;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Certificate error: {0}")]
    Certificate(#[from] CertificateError),

    #[error("Failed to send message. Error: {0}")]
    Sending(#[from] SendError),
}

#[derive(Debug, Error)]
pub enum CertificateError {
    #[error("Certificate is dated past garbage collection round. Certificate round: ({certificate_round}), GC round: {gc_round}")]
    Stale {
        certificate_round: Round,
        gc_round: Round,
    },
}

#[derive(Debug, Error)]
pub enum SendError {
    #[error("Failed to send certificate to the consensus layer. Error: {0}")]
    CertificateNew(#[from] futures::channel::mpsc::SendError),
}
