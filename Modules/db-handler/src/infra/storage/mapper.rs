use crate::infra::storage::entity;
use {{ crate_name }}_sdk::Product;

/// Convert a database entity to a contract model (owned version)
impl From<entity::product::Model> for Product {
    fn from(e: entity::product::Model) -> Self {
        Self {
            id: e.id,
            tenant_id: e.tenant_id,
            name: e.name,
            price: e.price,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

/// Convert a database entity to a contract model (by-ref version)
impl From<&entity::product::Model> for Product {
    fn from(e: &entity::product::Model) -> Self {
        Self {
            id: e.id,
            tenant_id: e.tenant_id,
            name: e.name.clone(),
            price: e.price,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
