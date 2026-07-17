use std::sync::Arc;

use axum::Router;
use toolkit::api::OpenApiRegistry;

use crate::gear::ConcreteUserService;

mod users;

pub(crate) fn register_routes(
    router: Router,
    openapi: &dyn OpenApiRegistry,
    service: Arc<ConcreteUserService>,
) -> Router {
    let router = users::register_user_routes(router, openapi);
    router.layer(axum::Extension(service))
}
