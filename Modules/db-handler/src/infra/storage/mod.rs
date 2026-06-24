//! Infrastructure storage layer - database persistence and OData mapping.

pub mod entity;
pub mod mapper;
pub mod migrations;
#[cfg(feature = "odata")]
pub mod odata_mapper;

mod db;
mod product_sea_repo;

pub use product_sea_repo::OrmProductRepository;
