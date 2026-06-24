use thiserror::Error;

/// Internal domain errors for the `{{ project-name }}` gear.
///
/// These are not exposed to external consumers — `PublicApiItemLocalClient` maps
/// these to the public `PublicApiItemError` at the SDK boundary.
#[derive(Error, Debug)]
pub enum DomainError {
    /// An HTTP-level error occurred while fetching from the external API.
    #[error("HTTP error: {0}")]
    Http(String),

    /// The API response could not be deserialized.
    #[error("Failed to parse API response: {0}")]
    Parse(String),
}
