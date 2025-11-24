//! Error types for Raudikko

use thiserror::Error;

/// Errors that can occur when loading or using morphology
#[derive(Debug, Error)]
pub enum MorphologyError {
    #[error("Failed to find bundled morphology")]
    BundledNotFound,

    #[error("Failed to load morphology: {0}")]
    LoadError(#[from] std::io::Error),

    #[error("Invalid morphology format: {0}")]
    InvalidFormat(String),
}

/// Errors related to structure parsing and manipulation
#[derive(Debug, Error)]
pub enum StructureError {
    #[error("Empty structure")]
    Empty,

    #[error("Unknown structure code: {0}")]
    UnknownCode(char),
}
