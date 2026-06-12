//! Thiserror-backed API error definitions for pokemon.

use http::StatusCode;
use toolkit::api::canonical_prelude::{CanonicalError, Problem, resource_error};
use thiserror::Error;

#[resource_error("gts.hx.example2.pokemon.api.v1~")]
pub struct PokemonApiError;

/// Strongly-typed API error codes for RFC 9457 responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum ErrorCode {
    #[error("Pokemon not found")]
    PokemonNotFound,

    #[error("Validation error")]
    PokemonValidation,

    #[error("Internal database error")]
    PokemonInternalDatabase,

    #[error("Internal server error")]
    InternalServerError,
}

impl ErrorCode {
    #[must_use]
    pub const fn pokemon_not_found_v1() -> Self {
        Self::PokemonNotFound
    }

    #[must_use]
    pub const fn pokemon_validation_v1() -> Self {
        Self::PokemonValidation
    }

    #[must_use]
    pub const fn pokemon_internal_database_v1() -> Self {
        Self::PokemonInternalDatabase
    }

    #[must_use]
    pub const fn internal_server_error_v1() -> Self {
        Self::InternalServerError
    }

    #[must_use]
    pub const fn status(self) -> StatusCode {
        match self {
            Self::PokemonNotFound => StatusCode::NOT_FOUND,
            Self::PokemonValidation => StatusCode::UNPROCESSABLE_ENTITY,
            Self::PokemonInternalDatabase => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::PokemonNotFound => "Pokemon Not Found",
            Self::PokemonValidation => "Validation Error",
            Self::PokemonInternalDatabase => "Internal Database Error",
            Self::InternalServerError => "Internal Server Error",
        }
    }

    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::PokemonNotFound => "gts.hx.core.errors.err.v1~hx.example2.pokemon.not_found.v1",
            Self::PokemonValidation => {
                "gts.hx.core.errors.err.v1~hx.example2.pokemon.validation.v1"
            }
            Self::PokemonInternalDatabase => {
                "gts.hx.core.errors.err.v1~hx.example2.pokemon.internal_database.v1"
            }
            Self::InternalServerError => "gts.hx.core.errors.err.v1~hx.internal_server_error.v1",
        }
    }

    #[must_use]
    pub const fn type_url(self) -> &'static str {
        self.code()
    }

    pub fn as_canonical(self, detail: impl Into<String>) -> CanonicalError {
        let detail = detail.into();

        match self {
            Self::PokemonNotFound => PokemonApiError::not_found(detail.clone())
                .with_resource(detail)
                .create(),
            Self::PokemonValidation => PokemonApiError::invalid_argument()
                .with_field_violation("pokemon", detail, "VALIDATION_FAILED")
                .create(),
            Self::PokemonInternalDatabase => CanonicalError::internal(detail).create(),
            Self::InternalServerError => CanonicalError::internal(detail).create(),
        }
    }

    pub fn as_problem(self, detail: impl Into<String>) -> Problem {
        Problem::from(self.as_canonical(detail))
    }

    pub fn with_context(
        self,
        detail: impl Into<String>,
        instance: &str,
        trace_id: Option<String>,
    ) -> Problem {
        let mut problem = self.as_problem(detail).with_instance(instance);
        if let Some(tid) = trace_id {
            problem = problem.with_trace_id(tid);
        }
        problem
    }
}
