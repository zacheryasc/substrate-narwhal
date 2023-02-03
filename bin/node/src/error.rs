use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Service(#[from] sc_service::Error),

    #[error(transparent)]
    SubstrateCli(#[from] sc_cli::Error),

    #[error("failed to parse input. Error: {0}")]
    ParseError(String),
}

impl Error {
    pub fn substrate_cli(e: sc_cli::Error) -> Self {
        Self::from(e)
    }
}
