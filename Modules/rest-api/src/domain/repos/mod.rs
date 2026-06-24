use toolkit::async_trait;
use uuid::Uuid;

use {{ crate_name }}_sdk::User;

use crate::domain::error::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<User>, DomainError>;

    async fn get(&self, id: Uuid) -> Result<Option<User>, DomainError>;

    async fn create(&self, user: User) -> Result<User, DomainError>;

    async fn update(&self, user: User) -> Result<User, DomainError>;

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;

    async fn email_exists(&self, email: &str, except_id: Option<Uuid>) -> Result<bool, DomainError>;
}
