//! Object-safe client boundary for the `{{ project-name }}` gear.
//!
//! This API is designed for `ClientHub` registration as `Arc<dyn PublicApiItemClientV1>`.
//! Other gears obtain the client from `ClientHub`:
//!
//! ```ignore
//! use {{ crate_name }}_sdk::PublicApiItemClientV1;
//!
//! let client = hub.get::<dyn PublicApiItemClientV1>()?;
//! let public_api_item = client.fetch_random_public_api_item().await?;
//! ```

use async_trait::async_trait;

use crate::errors::PublicApiItemError;
use crate::models::PublicApiItem;

/// Object-safe client for inter-gear consumption via `ClientHub` (Version 1).
///
/// Registered by `{{ project-name }}` during init:
/// ```ignore
/// ctx.client_hub().register::<dyn PublicApiItemClientV1>(Arc::new(local_client));
/// ```
#[async_trait]
pub trait PublicApiItemClientV1: Send + Sync {
    /// Fetch a random public_api_item from the JSONPlaceholder.
    async fn fetch_random_public_api_item(&self) -> Result<PublicApiItem, PublicApiItemError>;
}
