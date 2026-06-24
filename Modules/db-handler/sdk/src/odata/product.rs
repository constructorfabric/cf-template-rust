//! OData filter field definitions for Product resources.

use toolkit_odata_macros::ODataFilterable;
use toolkit_sdk::odata::{FieldRef, Schema};
use time::OffsetDateTime;
use uuid::Uuid;

use toolkit_odata::filter::FilterField as _;

/// Product filterable fields schema.
#[derive(ODataFilterable)]
pub struct ProductQuery {
    #[odata(filter(kind = "Uuid"))]
    pub id: Uuid,

    #[odata(filter(kind = "String"))]
    pub name: String,

    #[odata(filter(kind = "DateTimeUtc"))]
    pub created_at: OffsetDateTime,
}

/// Type alias for the generated filter field enum.
pub use ProductQueryFilterField as ProductFilterField;

#[derive(Debug, Clone, Copy)]
pub struct ProductSchema;

impl Schema for ProductSchema {
    type Field = ProductFilterField;

    fn field_name(field: Self::Field) -> &'static str {
        field.name()
    }
}

pub const PRODUCT_ID: FieldRef<ProductSchema, Uuid> = FieldRef::new(ProductFilterField::Id);
pub const PRODUCT_NAME: FieldRef<ProductSchema, String> = FieldRef::new(ProductFilterField::Name);
pub const PRODUCT_CREATED_AT: FieldRef<ProductSchema, OffsetDateTime> =
    FieldRef::new(ProductFilterField::CreatedAt);
