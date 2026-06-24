use std::sync::Arc;

use toolkit::async_trait;
use uuid::Uuid;

use {{ crate_name }}_sdk::{NewUser, UpdateUserRequest, User, UserClientV1, UserError};

use crate::gear::ConcreteUserService;

pub struct UserLocalClient {
    service: Arc<ConcreteUserService>,
}

impl UserLocalClient {
    pub fn new(service: Arc<ConcreteUserService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl UserClientV1 for UserLocalClient {
    async fn list_users(&self) -> Result<Vec<User>, UserError> {
        self.service.list_users().await.map_err(UserError::from)
    }

    async fn get_user(&self, id: Uuid) -> Result<User, UserError> {
        self.service.get_user(id).await.map_err(UserError::from)
    }

    async fn create_user(&self, new_user: NewUser) -> Result<User, UserError> {
        self.service
            .create_user(new_user)
            .await
            .map_err(UserError::from)
    }

    async fn update_user(&self, req: UpdateUserRequest) -> Result<User, UserError> {
        self.service.update_user(req).await.map_err(UserError::from)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        self.service.delete_user(id).await.map_err(UserError::from)
    }
}
