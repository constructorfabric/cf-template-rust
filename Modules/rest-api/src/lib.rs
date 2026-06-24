#![doc = include_str!("../README.md")]

pub mod gear;
pub use gear::UserRestApiGear;

pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod domain;
pub(crate) mod infra;
