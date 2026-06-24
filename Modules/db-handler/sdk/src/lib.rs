//! Product SDK
//!
//! Public API contract for the product module:
//! - `ProductClientV1` trait
//! - `Product` model
//! - `ProductError` error type
//! - OData filter schemas (behind `odata` feature)

pub mod client;
pub mod errors;
pub mod models;

#[cfg(feature = "odata")]
pub mod odata;

pub use client::ProductClientV1;
#[cfg(feature = "odata")]
pub use client::ProductStreamingClientV1;
pub use errors::ProductError;
pub use models::{NewProduct, Product, ProductPatch, UpdateProductRequest};
