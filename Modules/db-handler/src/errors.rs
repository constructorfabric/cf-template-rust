//! Thiserror-backed API error definitions for product.

use http::StatusCode;
use toolkit::api::canonical_prelude::{CanonicalError, Problem, resource_error};
use thiserror::Error;

#[resource_error("gts.hx.example2.product.api.v1~")]
pub struct ProductApiError;

/// Strongly-typed API error codes for RFC 9457 responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum ErrorCode {
    #[error("Product not found")]
    ProductNotFound,

    #[error("Validation error")]
    ProductValidation,

    #[error("Internal database error")]
    ProductInternalDatabase,

    #[error("Internal server error")]
    InternalServerError,
}

impl ErrorCode {
    #[must_use]
    pub const fn product_not_found_v1() -> Self {
        Self::ProductNotFound
    }

    #[must_use]
    pub const fn product_validation_v1() -> Self {
        Self::ProductValidation
    }

    #[must_use]
    pub const fn product_internal_database_v1() -> Self {
        Self::ProductInternalDatabase
    }

    #[must_use]
    pub const fn internal_server_error_v1() -> Self {
        Self::InternalServerError
    }

    #[must_use]
    pub const fn status(self) -> StatusCode {
        match self {
            Self::ProductNotFound => StatusCode::NOT_FOUND,
            Self::ProductValidation => StatusCode::UNPROCESSABLE_ENTITY,
            Self::ProductInternalDatabase => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::ProductNotFound => "Product Not Found",
            Self::ProductValidation => "Validation Error",
            Self::ProductInternalDatabase => "Internal Database Error",
            Self::InternalServerError => "Internal Server Error",
        }
    }

    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::ProductNotFound => "gts.hx.core.errors.err.v1~hx.example2.product.not_found.v1",
            Self::ProductValidation => {
                "gts.hx.core.errors.err.v1~hx.example2.product.validation.v1"
            }
            Self::ProductInternalDatabase => {
                "gts.hx.core.errors.err.v1~hx.example2.product.internal_database.v1"
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
            Self::ProductNotFound => ProductApiError::not_found(detail.clone())
                .with_resource(detail)
                .create(),
            Self::ProductValidation => ProductApiError::invalid_argument()
                .with_field_violation("product", detail, "VALIDATION_FAILED")
                .create(),
            Self::ProductInternalDatabase => CanonicalError::internal(detail).create(),
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
