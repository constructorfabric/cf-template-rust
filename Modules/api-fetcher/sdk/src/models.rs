//! Public models for the `{{ project-name }}` module.
//!
//! Transport-agnostic data structures that define the contract
//! between the `{{ project-name }}` module and its consumers.

/// A public_api_item entity fetched from the JSONPlaceholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicApiItem {
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}
