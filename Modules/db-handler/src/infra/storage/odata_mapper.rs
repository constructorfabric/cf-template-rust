//! Infrastructure layer mapping from type-safe `FilterNode` to SeaORM Conditions.
//! Only compiled when the `odata` feature is enabled.
use toolkit_db::odata::sea_orm_filter::{FieldToColumn, ODataFieldMapping};

use crate::infra::storage::entity::{Column, Entity, Model};
use {{ crate_name }}_sdk::odata::ProductFilterField;

/// Complete OData mapper for product.
pub struct ProductODataMapper;

impl FieldToColumn<ProductFilterField> for ProductODataMapper {
    type Column = Column;

    fn map_field(field: ProductFilterField) -> Column {
        match field {
            ProductFilterField::Id => Column::Id,
            ProductFilterField::Name => Column::Name,
            ProductFilterField::CreatedAt => Column::CreatedAt,
        }
    }
}

impl ODataFieldMapping<ProductFilterField> for ProductODataMapper {
    type Entity = Entity;

    fn extract_cursor_value(model: &Model, field: ProductFilterField) -> sea_orm::Value {
        match field {
            ProductFilterField::Id => sea_orm::Value::Uuid(Some(Box::new(model.id))),
            ProductFilterField::Name => sea_orm::Value::String(Some(Box::new(model.name.clone()))),
            ProductFilterField::CreatedAt => {
                sea_orm::Value::TimeDateTimeWithTimeZone(Some(Box::new(model.created_at)))
            }
        }
    }
}
