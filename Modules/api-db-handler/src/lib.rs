#![doc = include_str!("../README.md")]

// === API ERROR DEFINITIONS ===
pub mod errors;

// === MODULE DEFINITION ===
pub mod gear;
pub use gear::PokemonGear;

// === INTERNAL MODULES ===
pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod domain;
pub(crate) mod infra;
