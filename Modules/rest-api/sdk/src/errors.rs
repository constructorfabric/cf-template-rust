//! Public user API errors.

use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Clone)]
pub enum UserError {
    #[error("User not found: {id}")]
    NotFound { id: Uuid },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("User with email '{email}' already exists")]
    Conflict { email: String },

    #[error("Internal error")]
    Internal,
}

impl UserError {
    #[must_use]
    pub fn not_found(id: Uuid) -> Self {
        Self::NotFound { id }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn conflict(email: impl Into<String>) -> Self {
        Self::Conflict {
            email: email.into(),
        }
    }

    #[must_use]
    pub fn internal() -> Self {
        Self::Internal
    }
}
