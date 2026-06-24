use toolkit::api::canonical_prelude::{CanonicalError, Problem, resource_error};

use crate::domain::error::DomainError;

#[resource_error("gts.hx.example.rest_api.users.api.v1~")]
pub struct UsersApiError;

fn domain_error_to_canonical(error: &DomainError) -> CanonicalError {
    match error {
        DomainError::NotFound { id } => UsersApiError::not_found(format!("User {id} not found"))
            .with_resource(id.to_string())
            .create(),
        DomainError::Validation { field, message } => UsersApiError::invalid_argument()
            .with_field_violation(field.clone(), message.clone(), "VALIDATION_FAILED")
            .create(),
        DomainError::Conflict { email } => UsersApiError::already_exists(format!(
            "User with email '{email}' already exists"
        ))
        .with_resource(email.clone())
        .create(),
    }
}

impl From<DomainError> for CanonicalError {
    fn from(value: DomainError) -> Self {
        domain_error_to_canonical(&value)
    }
}

impl From<DomainError> for Problem {
    fn from(value: DomainError) -> Self {
        Problem::from(domain_error_to_canonical(&value))
    }
}
