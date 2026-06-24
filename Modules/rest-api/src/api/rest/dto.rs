use time::OffsetDateTime;
use uuid::Uuid;

use {{ crate_name }}_sdk::{NewUser, User, UserPatch};

#[derive(Debug, Clone)]
#[toolkit_macros::api_dto(request, response)]
pub struct UserDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub display_name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
#[toolkit_macros::api_dto(response)]
pub struct UserListResponse {
    pub items: Vec<UserDto>,
}

#[derive(Debug, Clone)]
#[toolkit_macros::api_dto(request)]
pub struct CreateUserReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub tenant_id: Uuid,
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Default)]
#[toolkit_macros::api_dto(request)]
pub struct UpdateUserReq {
    pub email: Option<String>,
    pub display_name: Option<String>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            email: user.email,
            display_name: user.display_name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<CreateUserReq> for NewUser {
    fn from(req: CreateUserReq) -> Self {
        Self {
            id: req.id,
            tenant_id: req.tenant_id,
            email: req.email,
            display_name: req.display_name,
        }
    }
}

impl From<UpdateUserReq> for UserPatch {
    fn from(req: UpdateUserReq) -> Self {
        Self {
            email: req.email,
            display_name: req.display_name,
        }
    }
}
