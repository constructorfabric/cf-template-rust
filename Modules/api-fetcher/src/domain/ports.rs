use crate::domain::error::DomainError;
use {{ crate_name }}_sdk::PublicApiItem;
use toolkit::async_trait;

/// Repository port for fetching PublicApiItem data.
///
/// Implemented by `PublicApiItemHttpRepository` in the infra layer.
#[async_trait]
pub trait PublicApiItemRepository: Send + Sync {
    async fn fetch_random(&self) -> Result<PublicApiItem, DomainError>;
}
