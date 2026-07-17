//! Object-safe client boundary for the `{{ project-name }}` gear.
//!
//! This API is designed for `ClientHub` registration as `Arc<dyn PokemonClientV1>`.
//! Other gears obtain the client from `ClientHub`:
//!
//! ```ignore
//! use {{ crate_name }}_sdk::PokemonClientV1;
//!
//! let client = hub.get::<dyn PokemonClientV1>()?;
//! let pokemon = client.fetch_random_pokemon().await?;
//! ```

use async_trait::async_trait;

use crate::errors::PokemonError;
use crate::models::Pokemon;

/// Object-safe client for inter-gear consumption via `ClientHub` (Version 1).
///
/// Registered by `{{ project-name }}` during init:
/// ```ignore
/// ctx.client_hub().register::<dyn PokemonClientV1>(Arc::new(local_client));
/// ```
#[async_trait]
pub trait PokemonClientV1: Send + Sync {
    /// Fetch a random pokemon from the PokeAPI.
    async fn fetch_random_pokemon(&self) -> Result<Pokemon, PokemonError>;
}
