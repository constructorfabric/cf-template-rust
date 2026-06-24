#![doc = include_str!("../README.md")]

// === API ERROR DEFINITIONS ===
pub mod errors;

// === GEAR DEFINITION ===
pub mod gear;
pub use gear::ProductGear;

// === INTERNAL CRATE MODULES ===
pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod domain;
pub(crate) mod infra;
