use axum::Router;
use toolkit::api::OpenApiRegistry;
use toolkit::api::operation_builder::OperationBuilder;

use super::super::{dto, handlers};

const API_TAG: &str = "Users";

pub(super) fn register_user_routes(mut router: Router, openapi: &dyn OpenApiRegistry) -> Router {
    router = OperationBuilder::get("/{{ project-name }}/v1/users")
        .operation_id("{{ crate_name }}.users.list")
        .summary("List users")
        .description("List all in-memory users")
        .tag(API_TAG)
        .public()
        .handler(handlers::list_users)
        .json_response_with_schema::<dto::UserListResponse>(
            openapi,
            http::StatusCode::OK,
            "Users",
        )
        .error_500(openapi)
        .register(router, openapi);

    router = OperationBuilder::get("/{{ project-name }}/v1/users/{id}")
        .operation_id("{{ crate_name }}.users.get")
        .summary("Get user")
        .description("Get a user by UUID")
        .tag(API_TAG)
        .public()
        .path_param("id", "User UUID")
        .handler(handlers::get_user)
        .json_response_with_schema::<dto::UserDto>(openapi, http::StatusCode::OK, "User")
        .error_404(openapi)
        .error_500(openapi)
        .register(router, openapi);

    router = OperationBuilder::post("/{{ project-name }}/v1/users")
        .operation_id("{{ crate_name }}.users.create")
        .summary("Create user")
        .description("Create an in-memory user")
        .tag(API_TAG)
        .public()
        .json_request::<dto::CreateUserReq>(openapi, "User creation data")
        .handler(handlers::create_user)
        .json_response_with_schema::<dto::UserDto>(
            openapi,
            http::StatusCode::CREATED,
            "Created user",
        )
        .error_400(openapi)
        .error_409(openapi)
        .error_500(openapi)
        .register(router, openapi);

    router = OperationBuilder::patch("/{{ project-name }}/v1/users/{id}")
        .operation_id("{{ crate_name }}.users.update")
        .summary("Update user")
        .description("Partially update an in-memory user")
        .tag(API_TAG)
        .public()
        .path_param("id", "User UUID")
        .json_request::<dto::UpdateUserReq>(openapi, "User update data")
        .handler(handlers::update_user)
        .json_response_with_schema::<dto::UserDto>(openapi, http::StatusCode::OK, "Updated user")
        .error_400(openapi)
        .error_404(openapi)
        .error_409(openapi)
        .error_500(openapi)
        .register(router, openapi);

    router = OperationBuilder::delete("/{{ project-name }}/v1/users/{id}")
        .operation_id("{{ crate_name }}.users.delete")
        .summary("Delete user")
        .description("Delete an in-memory user by UUID")
        .tag(API_TAG)
        .public()
        .path_param("id", "User UUID")
        .handler(handlers::delete_user)
        .json_response(http::StatusCode::NO_CONTENT, "User deleted")
        .error_404(openapi)
        .error_500(openapi)
        .register(router, openapi);

    router
}
