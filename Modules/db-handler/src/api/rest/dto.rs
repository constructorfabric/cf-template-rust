use {{ crate_name }}_sdk::{NewProduct, Product, ProductPatch};
use time::OffsetDateTime;
use uuid::Uuid;

/// REST DTO for product representation with serde/utoipa
#[derive(Debug, Clone)]
#[toolkit_macros::api_dto(request, response)]
pub struct ProductDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    /// Price in cents.
    pub price: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

/// REST DTO for creating a new product.
#[derive(Debug, Clone)]
#[toolkit_macros::api_dto(request)]
pub struct CreateProductReq {
    /// Optional ID for the product. If not provided, a UUID v7 will be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub tenant_id: Uuid,
    pub name: String,
    /// Price in cents.
    pub price: i32,
}

/// REST DTO for partially updating a product.
#[derive(Debug, Clone, Default)]
#[toolkit_macros::api_dto(request)]
pub struct UpdateProductReq {
    pub name: Option<String>,
    pub price: Option<i32>,
}

impl From<Product> for ProductDto {
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            tenant_id: p.tenant_id,
            name: p.name,
            price: p.price,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

impl From<CreateProductReq> for NewProduct {
    fn from(req: CreateProductReq) -> Self {
        Self {
            id: req.id,
            tenant_id: req.tenant_id,
            name: req.name,
            price: req.price,
        }
    }
}

impl From<UpdateProductReq> for ProductPatch {
    fn from(req: UpdateProductReq) -> Self {
        Self {
            name: req.name,
            price: req.price,
        }
    }
}
