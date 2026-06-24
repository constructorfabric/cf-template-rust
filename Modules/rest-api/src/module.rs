use std::sync::{Arc, OnceLock};

use toolkit::api::OpenApiRegistry;
use toolkit::{Gear, GearCtx, RestApiCapability, async_trait};
use tracing::info;

use {{ crate_name }}_sdk::UserClientV1;

use crate::api::rest::routes;
use crate::config::UserRestApiConfig;
use crate::domain::local_client::UserLocalClient;
use crate::domain::service::UserService;
use crate::infra::InMemoryUserRepository;

pub(crate) type ConcreteUserService = UserService<InMemoryUserRepository>;

#[toolkit::gear(
    name = "{{ project-name }}",
    capabilities = [rest]
)]
pub struct UserRestApiModule {
    service: OnceLock<Arc<ConcreteUserService>>,
}

impl Default for UserRestApiModule {
    fn default() -> Self {
        Self {
            service: OnceLock::new(),
        }
    }
}

#[async_trait]
impl Gear for UserRestApiModule {
    async fn init(&self, ctx: &GearCtx) -> anyhow::Result<()> {
        let _cfg: UserRestApiConfig = ctx.config()?;
        let service = Arc::new(UserService::new(Arc::new(InMemoryUserRepository::new())));
        let local_client = UserLocalClient::new(Arc::clone(&service));

        self.service
            .set(service)
            .map_err(|_| anyhow::anyhow!("{} module already initialized", Self::MODULE_NAME))?;

        ctx.client_hub()
            .register::<dyn UserClientV1>(Arc::new(local_client));

        info!("{{ project-name }} registered UserClientV1 into ClientHub");
        Ok(())
    }
}

impl RestApiCapability for UserRestApiModule {
    fn register_rest(
        &self,
        _ctx: &GearCtx,
        router: axum::Router,
        openapi: &dyn OpenApiRegistry,
    ) -> anyhow::Result<axum::Router> {
        let service = self
            .service
            .get()
            .ok_or_else(|| anyhow::anyhow!("service not initialized"))?
            .clone();

        Ok(routes::register_routes(router, openapi, service))
    }
}
