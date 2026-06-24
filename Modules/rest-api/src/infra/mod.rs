use std::collections::HashMap;

use parking_lot::RwLock;
use toolkit::async_trait;
use uuid::Uuid;

use {{ crate_name }}_sdk::User;

use crate::domain::error::DomainError;
use crate::domain::repos::UserRepository;

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: RwLock<HashMap<Uuid, User>>,
}

impl InMemoryUserRepository {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn list(&self) -> Result<Vec<User>, DomainError> {
        let mut users = self.users.read().values().cloned().collect::<Vec<_>>();
        users.sort_by_key(|user| user.created_at);
        Ok(users)
    }

    async fn get(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        Ok(self.users.read().get(&id).cloned())
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        self.users.write().insert(user.id, user.clone());
        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        self.users.write().insert(user.id, user.clone());
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        Ok(self.users.write().remove(&id).is_some())
    }

    async fn email_exists(&self, email: &str, except_id: Option<Uuid>) -> Result<bool, DomainError> {
        Ok(self
            .users
            .read()
            .values()
            .any(|user| user.email == email && Some(user.id) != except_id))
    }
}
