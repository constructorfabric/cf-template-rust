use serde::{Deserialize, Serialize};

/// Raw JSONPlaceholder response shape. Kept internal to the infra layer.
/// Mapped to `{{ crate_name }}_sdk::PublicApiItem` before crossing the domain boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicApiItemResponse {
    #[serde(rename = "userId")]
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}
