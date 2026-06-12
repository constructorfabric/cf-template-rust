use crate::domain::error::DomainError;
use {{ crate_name }}_sdk::Pokemon;
use toolkit::async_trait;

/// Repository port for fetching Pokemon data.
///
/// Implemented by `PokemonHttpRepository` in the infra layer.
#[async_trait]
pub trait PokemonRepository: Send + Sync {
    async fn fetch_random(&self) -> Result<Pokemon, DomainError>;
}
