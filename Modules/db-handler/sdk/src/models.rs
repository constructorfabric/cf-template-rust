//! Public models for the product module.

use time::OffsetDateTime;
use uuid::Uuid;

/// A product entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    /// Price in cents.
    pub price: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Data for creating a new product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewProduct {
    pub id: Option<Uuid>,
    pub tenant_id: Uuid,
    pub name: String,
    /// Price in cents.
    pub price: i32,
}

/// Partial update data for a product.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProductPatch {
    pub name: Option<String>,
    pub price: Option<i32>,
}

/// Request to update a product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateProductRequest {
    pub id: Uuid,
    pub patch: ProductPatch,
}
