//! Object-safe streaming boundary for the product module.
//!
//! Designed for `ClientHub` registration as `Arc<dyn ProductClientV1>`.

#[cfg(feature = "odata")]
use futures_core::Stream;
use toolkit::async_trait;
#[cfg(feature = "odata")]
use toolkit_sdk::odata::QueryBuilder;
#[cfg(feature = "odata")]
use std::pin::Pin;
use uuid::Uuid;

use crate::errors::ProductError;
use crate::models::{NewProduct, Product, UpdateProductRequest};

#[cfg(feature = "odata")]
use crate::odata::ProductSchema;

/// Boxed stream type returned by streaming client facades.
#[cfg(feature = "odata")]
pub type ProductStream<T> = Pin<Box<dyn Stream<Item = Result<T, ProductError>> + Send + 'static>>;

/// Object-safe client for inter-module consumption via `ClientHub` (Version 1).
#[async_trait]
pub trait ProductClientV1: Send + Sync {
    #[cfg(feature = "odata")]
    fn product(&self) -> Box<dyn ProductStreamingClientV1>;

    /// Get a single product by ID.
    async fn get_product(&self, id: Uuid) -> Result<Product, ProductError>;

    /// List product with cursor-based pagination.
    async fn list_product(
        &self,
        query: toolkit_odata::ODataQuery,
    ) -> Result<toolkit_odata::Page<Product>, ProductError>;

    /// Create a new product.
    async fn create_product(&self, new_product: NewProduct) -> Result<Product, ProductError>;

    /// Update an existing product.
    async fn update_product(&self, req: UpdateProductRequest) -> Result<Product, ProductError>;

    /// Delete a product by ID.
    async fn delete_product(&self, id: Uuid) -> Result<(), ProductError>;
}

/// Streaming interface for product (Version 1).
#[cfg(feature = "odata")]
pub trait ProductStreamingClientV1: Send + Sync {
    fn stream(&self, query: QueryBuilder<ProductSchema>) -> ProductStream<Product>;
}
