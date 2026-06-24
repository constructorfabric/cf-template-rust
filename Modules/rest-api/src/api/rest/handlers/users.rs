use axum::Extension;
use axum::extract::Path;
use axum::http::Uri;
use axum::response::IntoResponse;
use uuid::Uuid;

use {{ crate_name }}_sdk::UpdateUserRequest;

use super::{
    ApiResult, CreateUserReq, Json, JsonBody, UpdateUserReq, UserDto, UserListResponse,
    created_json, no_content,
};
use crate::module::ConcreteUserService;

pub async fn list_users(
    Extension(service): Extension<std::sync::Arc<ConcreteUserService>>,
) -> ApiResult<JsonBody<UserListResponse>> {
    let users = service
        .list_users()
        .await?
        .into_iter()
        .map(UserDto::from)
        .collect();

    Ok(Json(UserListResponse { items: users }))
}

pub async fn get_user(
    Extension(service): Extension<std::sync::Arc<ConcreteUserService>>,
    Path(id): Path<Uuid>,
) -> ApiResult<JsonBody<UserDto>> {
    let user = service.get_user(id).await?;
    Ok(Json(UserDto::from(user)))
}

pub async fn create_user(
    uri: Uri,
    Extension(service): Extension<std::sync::Arc<ConcreteUserService>>,
    Json(req_body): Json<CreateUserReq>,
) -> ApiResult<impl IntoResponse> {
    let user = service.create_user(req_body.into()).await?;
    let id = user.id.to_string();
    Ok(created_json(UserDto::from(user), &uri, &id).into_response())
}

pub async fn update_user(
    Extension(service): Extension<std::sync::Arc<ConcreteUserService>>,
    Path(id): Path<Uuid>,
    Json(req_body): Json<UpdateUserReq>,
) -> ApiResult<JsonBody<UserDto>> {
    let user = service
        .update_user(UpdateUserRequest {
            id,
            patch: req_body.into(),
        })
        .await?;

    Ok(Json(UserDto::from(user)))
}

pub async fn delete_user(
    Extension(service): Extension<std::sync::Arc<ConcreteUserService>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    service.delete_user(id).await?;
    Ok(no_content().into_response())
}
