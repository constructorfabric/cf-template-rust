//! Public models for the `{{ project-name }}` gear.
//!
//! Transport-agnostic data structures that define the contract
//! between the `{{ project-name }}` gear and its consumers.

/// A pokemon entity fetched from the PokeAPI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    pub id: u32,
    pub height: u32,
    pub name: String,
}
