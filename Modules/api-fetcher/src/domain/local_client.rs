use std::sync::Arc;

use {{ crate_name }}_sdk::{PublicApiItem, PublicApiItemClientV1, PublicApiItemError};
use toolkit::async_trait;

use crate::domain::service::PublicApiItemService;

/// SDK boundary adapter: implements `PublicApiItemClientV1` by delegating to `PublicApiItemService`.
///
/// Registered into `ClientHub` during module `init()` so other modules can call:
/// ```ignore
/// let client = hub.get::<dyn PublicApiItemClientV1>()?;
/// let public_api_item = client.fetch_random_public_api_item().await?;
/// ```
pub struct PublicApiItemLocalClient {
    service: Arc<PublicApiItemService>,
}

impl PublicApiItemLocalClient {
    pub fn new(service: Arc<PublicApiItemService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl PublicApiItemClientV1 for PublicApiItemLocalClient {
    async fn fetch_random_public_api_item(&self) -> Result<PublicApiItem, PublicApiItemError> {
        self.service.fetch_random_public_api_item().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to fetch random public_api_item");
            PublicApiItemError::internal("internal server error")
        })
    }
}
