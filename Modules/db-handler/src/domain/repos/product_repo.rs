use {{ crate_name }}_sdk::Product;
use toolkit::async_trait;
use toolkit_db::secure::DBRunner;
use toolkit_odata::{ODataQuery, Page};
use toolkit_security::AccessScope;
use uuid::Uuid;

use crate::domain::error::DomainError;

/// Repository trait for Product persistence operations.
#[async_trait]
pub trait ProductRepository: Send + Sync {
    /// Find a product by ID within the given security scope.
    async fn get<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<Option<Product>, DomainError>;

    /// List product with cursor-based pagination and OData filtering.
    async fn list_page<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        query: &ODataQuery,
    ) -> Result<Page<Product>, DomainError>;

    /// Create a new product.
    async fn create<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        product: Product,
    ) -> Result<Product, DomainError>;

    /// Update an existing product.
    async fn update<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        product: Product,
    ) -> Result<Product, DomainError>;

    /// Delete a product by ID.
    async fn delete<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<bool, DomainError>;

    /// Check whether a product exists.
    async fn exists<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<bool, DomainError>;
}
