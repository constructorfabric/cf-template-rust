use std::sync::Arc;

use {{ crate_name }}_sdk::PublicApiItem;

use crate::domain::error::DomainError;
use crate::domain::ports::PublicApiItemRepository;

/// Domain service that orchestrates PublicApiItem fetching.
///
/// Delegates to the `PublicApiItemRepository` port (implemented by `PublicApiItemHttpRepository`).
pub struct PublicApiItemService {
    repository: Arc<dyn PublicApiItemRepository>,
}

impl PublicApiItemService {
    pub fn new(repository: Arc<dyn PublicApiItemRepository>) -> Self {
        Self { repository }
    }

    pub async fn fetch_random_public_api_item(&self) -> Result<PublicApiItem, DomainError> {
        self.repository.fetch_random().await
    }
}
