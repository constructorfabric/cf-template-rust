#![doc = include_str!("../README.md")]

pub mod module;
pub use module::UserRestApiModule;

pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod domain;
pub(crate) mod infra;
