//! {{ project-name }} SDK

pub mod client;
pub mod errors;
pub mod models;

pub use client::UserClientV1;
pub use errors::UserError;
pub use models::{NewUser, UpdateUserRequest, User, UserPatch};
