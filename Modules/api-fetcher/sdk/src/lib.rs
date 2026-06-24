//! {{ project-name }} SDK
//!
//! This crate provides the public API for the `{{ project-name }}` module:
//! - `PublicApiItemClientV1` trait — the inter-module client interface
//! - `PublicApiItem` — the public model type
//! - `PublicApiItemError` — the public error type
//!
//! ## Usage
//!
//! Consumers obtain the client from `ClientHub`:
//! ```ignore
//! use {{ crate_name }}_sdk::PublicApiItemClientV1;
//!
//! // Get the client from ClientHub
//! let client = hub.get::<dyn PublicApiItemClientV1>()?;
//!
//! // Use the API
//! let public_api_item = client.fetch_random_public_api_item().await?;
//! println!("Got: {} (title={})", public_api_item.name, public_api_item.title);
//! ```

pub mod client;
pub mod errors;
pub mod models;

// Re-export main types at crate root for convenience
pub use client::PublicApiItemClientV1;
pub use errors::PublicApiItemError;
pub use models::PublicApiItem;
