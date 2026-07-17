use thiserror::Error;
use uuid::Uuid;

use {{ crate_name }}_sdk::UserError;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User not found: {id}")]
    NotFound { id: Uuid },

    #[error("Validation failed: {field}: {message}")]
    Validation { field: String, message: String },

    #[error("User with email '{email}' already exists")]
    Conflict { email: String },
}

impl DomainError {
    #[must_use]
    pub fn not_found(id: Uuid) -> Self {
        Self::NotFound { id }
    }

    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn conflict(email: impl Into<String>) -> Self {
        Self::Conflict {
            email: email.into(),
        }
    }
}

impl From<DomainError> for UserError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::NotFound { id } => UserError::not_found(id),
            DomainError::Validation { field, message } => {
                UserError::validation(format!("{field}: {message}"))
            }
            DomainError::Conflict { email } => UserError::conflict(email),
        }
    }
}
