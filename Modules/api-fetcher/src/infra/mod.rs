use anyhow::Context;
use toolkit::async_trait;
use toolkit_http::HttpClient;
use std::time::{Duration, UNIX_EPOCH};

use {{ crate_name }}_sdk::PublicApiItem;

use crate::domain::error::DomainError;
use crate::domain::ports::PublicApiItemRepository;

mod model;
use model::PublicApiItemResponse;

const API_URL: &str = "https://jsonplaceholder.typicode.com/posts/";

/// Infra implementation of `PublicApiItemRepository` — fetches PublicApiItem over HTTP from JSONPlaceholder.
pub struct PublicApiItemHttpRepository {
    client: HttpClient,
}

impl PublicApiItemHttpRepository {
    pub fn new() -> toolkit::Result<Self> {
        Ok(Self {
            client: HttpClient::builder()
                .no_redirects()
                .timeout(Duration::from_secs(5))
                .build()
                .context("problem while building http client")?,
        })
    }
}

#[async_trait]
impl PublicApiItemRepository for PublicApiItemHttpRepository {
    async fn fetch_random(&self) -> Result<PublicApiItem, DomainError> {
        let url = format!(
            "{}{}",
            API_URL,
            (UNIX_EPOCH
                .elapsed()
                .map_err(|e| DomainError::Http(e.to_string()))?
                .subsec_nanos()
                % 100)
                + 1
        );
        tracing::debug!("Fetching public_api_item from: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Http(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::Http(format!(
                "HTTP {} from {}: {}",
                response.status(),
                url,
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            )));
        }

        let raw: PublicApiItemResponse = response
            .json()
            .await
            .map_err(|e| DomainError::Parse(e.to_string()))?;

        tracing::info!("Successfully fetched public_api_item: {:?}", raw);

        Ok(PublicApiItem {
            user_id: raw.user_id,
            id: raw.id,
            title: raw.title,
            body: raw.body,
        })
    }
}
