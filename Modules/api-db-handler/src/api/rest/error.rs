use toolkit::api::canonical_prelude::{CanonicalError, Problem};

use crate::domain::error::DomainError;
use crate::errors::ErrorCode;

fn domain_error_to_canonical(e: &DomainError) -> CanonicalError {
    match e {
        DomainError::NotFound { id } => {
            ErrorCode::pokemon_not_found_v1().as_canonical(format!("Pokemon with id {id} was not found"))
        }
        DomainError::Validation { .. } => ErrorCode::pokemon_validation_v1().as_canonical(format!("{e}")),
        DomainError::Database { .. } => {
            tracing::error!(error = ?e, "Database error occurred");
            ErrorCode::pokemon_internal_database_v1().as_canonical("An internal database error occurred")
        }
        DomainError::InternalError => {
            tracing::error!(error = ?e, "Internal error occurred");
            ErrorCode::internal_server_error_v1().as_canonical("An internal error occurred")
        }
    }
}

/// Map domain error to RFC9457 Problem using thiserror-backed API error codes.
pub fn domain_error_to_problem(e: &DomainError, instance: &str) -> Problem {
    let trace_id = tracing::Span::current()
        .id()
        .map(|id| id.into_u64().to_string());

    let mut problem = Problem::from(domain_error_to_canonical(e)).with_instance(instance);
    if let Some(tid) = trace_id {
        problem = problem.with_trace_id(tid);
    }
    problem
}

impl From<DomainError> for CanonicalError {
    fn from(e: DomainError) -> Self {
        domain_error_to_canonical(&e)
    }
}

/// Implement Into<Problem> for `DomainError` so `?` works in handlers
impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        domain_error_to_problem(&e, "/")
    }
}
