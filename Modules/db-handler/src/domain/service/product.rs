use std::sync::Arc;

use {{ crate_name }}_sdk::{NewProduct, Product, ProductPatch};
use time::OffsetDateTime;
use toolkit_macros::domain_model;
use toolkit_odata::{ODataQuery, Page};
use toolkit_security::AccessScope;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::repos::ProductRepository;
use crate::domain::service::{DbProvider, ServiceConfig};

/// Product service.
#[domain_model]
pub struct ProductService<R: ProductRepository + 'static> {
    db: Arc<DbProvider>,
    repo: Arc<R>,
    #[allow(dead_code)] // in case we need it
    config: ServiceConfig,
}

impl<R: ProductRepository + 'static> ProductService<R> {
    pub fn new(db: Arc<DbProvider>, repo: Arc<R>, config: ServiceConfig) -> Self {
        Self { db, repo, config }
    }
}

impl<R: ProductRepository + 'static> ProductService<R> {
    #[instrument(skip(self), fields(product_id = %id))]
    pub async fn get_product(&self, id: Uuid) -> Result<Product, DomainError> {
        tracing::debug!("Getting product by id");

        let conn = self.db.conn().map_err(DomainError::from)?;
        // We are allowing all because the API is public atm, if you want authentication change this.
        let scope = AccessScope::allow_all();

        let product = self
            .repo
            .get(&conn, &scope, id)
            .await?
            .ok_or_else(|| DomainError::not_found(id))?;

        tracing::debug!("Successfully retrieved product");
        Ok(product)
    }

    /// List product with cursor-based pagination
    #[instrument(skip(self, query))]
    pub async fn list_product_page(
        &self,
        query: &ODataQuery,
    ) -> Result<Page<Product>, DomainError> {
        tracing::debug!("Listing product with cursor pagination");

        let conn = self.db.conn().map_err(DomainError::from)?;
        // We are allowing all because the API is public atm, if you want authentication change this.
        let scope = AccessScope::allow_all();

        let page = self.repo.list_page(&conn, &scope, query).await?;

        tracing::debug!("Successfully listed {} product in page", page.items.len());
        Ok(page)
    }

    #[instrument(skip(self, new_product), fields(product.name = %new_product.name))]
    pub async fn create_product(&self, new_product: NewProduct) -> Result<Product, DomainError> {
        self.validate_new_product(&new_product)?;

        let conn = self.db.conn().map_err(DomainError::from)?;
        let scope = AccessScope::allow_all();
        let id = new_product.id.unwrap_or_else(Uuid::now_v7);

        if new_product.id.is_some() && self.repo.exists(&conn, &scope, id).await? {
            return Err(DomainError::validation(
                "id",
                "Product with this ID already exists",
            ));
        }

        let now = OffsetDateTime::now_utc();
        let product = Product {
            id,
            tenant_id: new_product.tenant_id,
            name: new_product.name,
            price: new_product.price,
            created_at: now,
            updated_at: now,
        };

        self.repo.create(&conn, &scope, product).await
    }

    #[instrument(skip(self, patch), fields(product_id = %id))]
    pub async fn update_product(
        &self,
        id: Uuid,
        patch: ProductPatch,
    ) -> Result<Product, DomainError> {
        self.validate_product_patch(&patch)?;

        let conn = self.db.conn().map_err(DomainError::from)?;
        let scope = AccessScope::allow_all();
        let mut current = self
            .repo
            .get(&conn, &scope, id)
            .await?
            .ok_or_else(|| DomainError::not_found(id))?;

        if let Some(name) = patch.name {
            current.name = name;
        }
        if let Some(price) = patch.price {
            current.price = price;
        }
        current.updated_at = OffsetDateTime::now_utc();

        self.repo.update(&conn, &scope, current).await
    }

    #[instrument(skip(self), fields(product_id = %id))]
    pub async fn delete_product(&self, id: Uuid) -> Result<(), DomainError> {
        let conn = self.db.conn().map_err(DomainError::from)?;
        let scope = AccessScope::allow_all();
        let deleted = self.repo.delete(&conn, &scope, id).await?;

        if !deleted {
            return Err(DomainError::not_found(id));
        }

        Ok(())
    }

    fn validate_new_product(&self, product: &NewProduct) -> Result<(), DomainError> {
        if product.name.trim().is_empty() {
            return Err(DomainError::validation("name", "must not be empty"));
        }
        if product.price < 0 {
            return Err(DomainError::validation("price", "must be >= 0"));
        }
        Ok(())
    }

    fn validate_product_patch(&self, patch: &ProductPatch) -> Result<(), DomainError> {
        if let Some(name) = &patch.name
            && name.trim().is_empty()
        {
            return Err(DomainError::validation("name", "must not be empty"));
        }
        if let Some(price) = patch.price
            && price < 0
        {
            return Err(DomainError::validation("price", "must be >= 0"));
        }
        Ok(())
    }
}
