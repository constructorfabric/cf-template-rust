//! Object-safe ClientHub boundary for user CRUD operations.

use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::UserError;
use crate::models::{NewUser, UpdateUserRequest, User};

#[async_trait]
pub trait UserClientV1: Send + Sync {
    async fn list_users(&self) -> Result<Vec<User>, UserError>;

    async fn get_user(&self, id: Uuid) -> Result<User, UserError>;

    async fn create_user(&self, new_user: NewUser) -> Result<User, UserError>;

    async fn update_user(&self, req: UpdateUserRequest) -> Result<User, UserError>;

    async fn delete_user(&self, id: Uuid) -> Result<(), UserError>;
}
