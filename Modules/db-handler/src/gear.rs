use std::sync::{Arc, OnceLock};

use toolkit::api::OpenApiRegistry;
use toolkit::{DatabaseCapability, Gear, GearCtx, RestApiCapability, async_trait};
use toolkit_db::DBProvider;
use toolkit_db::DbError;
use sea_orm_migration::MigrationTrait;
use tracing::{debug, info};

use {{ crate_name }}_sdk::ProductClientV1;

use crate::api::rest::routes;
use crate::config::ProductConfig;
use crate::domain::local_client::client::ProductLocalClient;
use crate::domain::service::{AppServices, ServiceConfig};
use crate::infra::storage::OrmProductRepository;

/// Type alias for the concrete `AppServices` type used with ORM repositories.
pub(crate) type ConcreteAppServices = AppServices<OrmProductRepository>;

/// Product gear with DDD-light layout and proper `ClientHub` integration
#[toolkit::gear(
    name = "{{ project-name }}",
    capabilities = [db, rest]
)]
pub struct ProductGear {
    service: OnceLock<Arc<ConcreteAppServices>>,
}

impl Default for ProductGear {
    fn default() -> Self {
        Self {
            service: OnceLock::new(),
        }
    }
}

#[async_trait]
impl Gear for ProductGear {
    async fn init(&self, ctx: &GearCtx) -> anyhow::Result<()> {
        let cfg: ProductConfig = ctx.config()?;
        debug!(
            "Loaded product config: default_page_size={}, max_page_size={}",
            cfg.default_page_size, cfg.max_page_size
        );

        let db: Arc<DBProvider<DbError>> = Arc::new(ctx.db_required()?);

        let service_config = ServiceConfig {
            default_page_size: cfg.default_page_size,
            max_page_size: cfg.max_page_size,
        };

        let limit_cfg = service_config.limit_cfg();
        let product_repo = OrmProductRepository::new(limit_cfg);

        let services = Arc::new(AppServices::new(product_repo, db, service_config));

        self.service
            .set(services.clone())
            .map_err(|_| anyhow::anyhow!("{} gear already initialized", Self::MODULE_NAME))?;

        let local = ProductLocalClient::new(services);

        ctx.client_hub()
            .register::<dyn ProductClientV1>(Arc::new(local));

        Ok(())
    }
}

impl DatabaseCapability for ProductGear {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        use sea_orm_migration::MigratorTrait;
        info!("Providing product database migrations");
        crate::infra::storage::migrations::Migrator::migrations()
    }
}

impl RestApiCapability for ProductGear {
    fn register_rest(
        &self,
        _ctx: &GearCtx,
        router: axum::Router,
        openapi: &dyn OpenApiRegistry,
    ) -> anyhow::Result<axum::Router> {
        info!("Registering product REST routes");

        let service = self
            .service
            .get()
            .ok_or_else(|| anyhow::anyhow!("Service not initialized"))?
            .clone();

        let router = routes::register_routes(router, openapi, service);

        info!("Product REST routes registered successfully");
        Ok(router)
    }
}
