//! Public error types for the `{{ project-name }}` module.
//!
//! These errors are safe to expose to other modules and consumers.

use thiserror::Error;

/// Errors that can be returned by the `PublicApiItemClientV1`.
#[derive(Error, Debug, Clone)]
pub enum PublicApiItemError {
    /// An internal error occurred.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl PublicApiItemError {
    /// Create an `Internal` error with a message.
    #[must_use]
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
