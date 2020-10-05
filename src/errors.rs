use thiserror::Error;

#[derive(Error, Debug)]
pub enum TariError {
    /// Could not create a directory or file error
    #[error("Could not create a directory or file")]
    FailedCreatingFile(String),
    /// Invalid fuzzer option
    #[error("invalid fuzzer engine (expected {expected:?}, got {found:?})")]
    InvalidFuzzer { expected: String, found: String },
    #[error("Unexpected Error")]
    UnexpectedError(String),
}

/// Result type for Tari
pub type Result<T> = anyhow::Result<T, TariError>;
