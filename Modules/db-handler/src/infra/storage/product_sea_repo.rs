use toolkit::async_trait;

use crate::infra::storage::db::db_err;
use crate::infra::storage::entity::product::{
    ActiveModel as ProductActiveModel, Column, Entity as ProductEntity,
};
#[cfg(feature = "odata")]
use crate::infra::storage::odata_mapper::ProductODataMapper;
use crate::{domain::error::DomainError, domain::repos::ProductRepository};
use {{ crate_name }}_sdk::Product;
#[cfg(feature = "odata")]
use {{ crate_name }}_sdk::odata::ProductFilterField;
use toolkit_db::odata::LimitCfg;
#[cfg(feature = "odata")]
use toolkit_db::odata::paginate_odata;
use toolkit_db::secure::{DBRunner, SecureEntityExt};
#[cfg(feature = "odata")]
use toolkit_odata::SortDir;
use toolkit_odata::{ODataQuery, Page};
use toolkit_security::AccessScope;
use sea_orm::{EntityTrait, QueryFilter, Set};
use sea_orm::sea_query::Expr;
use toolkit_db::secure::{secure_insert, secure_update_with_scope, SecureDeleteExt};
use uuid::Uuid;

/// ORM-based implementation of the `ProductRepository` trait.
#[derive(Clone)]
pub struct OrmProductRepository {
    limit_cfg: LimitCfg,
}

impl OrmProductRepository {
    #[must_use]
    pub fn new(limit_cfg: LimitCfg) -> Self {
        Self { limit_cfg }
    }
}

#[async_trait]
impl ProductRepository for OrmProductRepository {
    async fn get<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<Option<Product>, DomainError> {
        let found = ProductEntity::find()
            .filter(sea_orm::Condition::all().add(Expr::col(Column::Id).eq(id)))
            .secure()
            .scope_with(scope)
            .one(conn)
            .await
            .map_err(db_err)?;
        Ok(found.map(Into::into))
    }

    async fn list_page<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        query: &ODataQuery,
    ) -> Result<Page<Product>, DomainError> {
        #[cfg(feature = "odata")]
        {
            let base_query = ProductEntity::find().secure().scope_with(scope);

            let page = paginate_odata::<ProductFilterField, ProductODataMapper, _, _, _, _>(
                base_query,
                conn,
                query,
                ("id", SortDir::Desc),
                self.limit_cfg,
                Into::into,
            )
            .await
            .map_err(db_err)?;

            Ok(page)
        }

        #[cfg(not(feature = "odata"))]
        {
            let _ = (conn, scope, query);
            Err(DomainError::validation(
                "query",
                "OData feature is disabled",
            ))
        }
    }

    async fn create<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        product: Product,
    ) -> Result<Product, DomainError> {
        let active = ProductActiveModel {
            id: Set(product.id),
            tenant_id: Set(product.tenant_id),
            name: Set(product.name.clone()),
            price: Set(product.price),
            created_at: Set(product.created_at),
            updated_at: Set(product.updated_at),
        };

        let _ = secure_insert::<ProductEntity>(active, scope, conn)
            .await
            .map_err(db_err)?;
        Ok(product)
    }

    async fn update<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        product: Product,
    ) -> Result<Product, DomainError> {
        let active = ProductActiveModel {
            id: Set(product.id),
            tenant_id: Set(product.tenant_id),
            name: Set(product.name.clone()),
            price: Set(product.price),
            created_at: Set(product.created_at),
            updated_at: Set(product.updated_at),
        };

        let _ = secure_update_with_scope::<ProductEntity>(active, scope, product.id, conn)
            .await
            .map_err(db_err)?;
        Ok(product)
    }

    async fn delete<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<bool, DomainError> {
        let result = ProductEntity::delete_many()
            .filter(sea_orm::Condition::all().add(Expr::col(Column::Id).eq(id)))
            .secure()
            .scope_with(scope)
            .exec(conn)
            .await
            .map_err(db_err)?;

        Ok(result.rows_affected > 0)
    }

    async fn exists<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<bool, DomainError> {
        let found = ProductEntity::find()
            .filter(sea_orm::Condition::all().add(Expr::col(Column::Id).eq(id)))
            .secure()
            .scope_with(scope)
            .one(conn)
            .await
            .map_err(db_err)?;

        Ok(found.is_some())
    }
}
