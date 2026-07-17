use std::sync::Arc;

use time::OffsetDateTime;
use toolkit_macros::domain_model;
use uuid::Uuid;

use {{ crate_name }}_sdk::{NewUser, UpdateUserRequest, User, UserPatch};

use crate::domain::error::DomainError;
use crate::domain::repos::UserRepository;

#[domain_model]
pub struct UserService<R: UserRepository + 'static> {
    repo: Arc<R>,
}

impl<R: UserRepository + 'static> UserService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, DomainError> {
        self.repo.list().await
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, DomainError> {
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| DomainError::not_found(id))
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, DomainError> {
        validate_new_user(&new_user)?;

        if self.repo.email_exists(&new_user.email, None).await? {
            return Err(DomainError::conflict(new_user.email));
        }

        let now = OffsetDateTime::now_utc();
        let user = User {
            id: new_user.id.unwrap_or_else(Uuid::now_v7),
            tenant_id: new_user.tenant_id,
            email: new_user.email,
            display_name: new_user.display_name,
            created_at: now,
            updated_at: now,
        };

        self.repo.create(user).await
    }

    pub async fn update_user(&self, req: UpdateUserRequest) -> Result<User, DomainError> {
        validate_patch(&req.patch)?;

        let mut current = self.get_user(req.id).await?;
        if let Some(email) = req.patch.email {
            if email != current.email && self.repo.email_exists(&email, Some(req.id)).await? {
                return Err(DomainError::conflict(email));
            }
            current.email = email;
        }
        if let Some(display_name) = req.patch.display_name {
            current.display_name = display_name;
        }
        current.updated_at = OffsetDateTime::now_utc();

        self.repo.update(current).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), DomainError> {
        if self.repo.delete(id).await? {
            Ok(())
        } else {
            Err(DomainError::not_found(id))
        }
    }
}

fn validate_new_user(user: &NewUser) -> Result<(), DomainError> {
    validate_email(&user.email)?;
    validate_display_name(&user.display_name)?;
    Ok(())
}

fn validate_patch(patch: &UserPatch) -> Result<(), DomainError> {
    if let Some(email) = &patch.email {
        validate_email(email)?;
    }
    if let Some(display_name) = &patch.display_name {
        validate_display_name(display_name)?;
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), DomainError> {
    if !email.contains('@') {
        return Err(DomainError::validation("email", "must contain @"));
    }
    Ok(())
}

fn validate_display_name(display_name: &str) -> Result<(), DomainError> {
    if display_name.trim().is_empty() {
        return Err(DomainError::validation("display_name", "must not be empty"));
    }
    Ok(())
}
