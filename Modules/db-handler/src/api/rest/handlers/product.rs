use axum::Extension;
use axum::extract::Path;
use axum::http::Uri;
use axum::response::IntoResponse;
use tracing::field::Empty;
use uuid::Uuid;

use toolkit::api::odata::OData;

use super::{
    ApiResult, CreateProductReq, Json, JsonBody, JsonPage, ProductDto, UpdateProductReq,
    apply_select, created_json, no_content, page_to_projected_json,
};
use crate::gear::ConcreteAppServices;

/// List product with cursor-based pagination and optional field projection via $select
#[tracing::instrument(
    skip(svc, query),
    fields(
        limit = query.limit,
        request_id = Empty,
    )
)]
pub async fn list_product(
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    OData(query): OData,
) -> ApiResult<JsonPage<serde_json::Value>> {
    let page = svc.product.list_product_page(&query).await?;
    let page = page.map_items(ProductDto::from);

    Ok(Json(page_to_projected_json(&page, query.selected_fields())))
}

/// Get a specific product by ID with optional field projection via $select
#[tracing::instrument(
    skip(svc),
    fields(
        product.id = %id,
        request_id = Empty,
    )
)]
pub async fn get_product(
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    Path(id): Path<Uuid>,
    OData(query): OData,
) -> ApiResult<JsonBody<serde_json::Value>> {
    let product = svc.product.get_product(id).await?;
    let product_dto = ProductDto::from(product);
    let projected = apply_select(&product_dto, query.selected_fields());
    Ok(Json(projected))
}

/// Create a new product.
#[tracing::instrument(
    skip(svc, req_body, uri),
    fields(
        product.name = %req_body.name,
        product.tenant_id = %req_body.tenant_id,
        request_id = Empty,
    )
)]
pub async fn create_product(
    uri: Uri,
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    Json(req_body): Json<CreateProductReq>,
) -> ApiResult<impl IntoResponse> {
    let product = svc.product.create_product(req_body.into()).await?;
    let id_str = product.id.to_string();
    Ok(created_json(ProductDto::from(product), &uri, &id_str).into_response())
}

/// Update a product by ID.
#[tracing::instrument(
    skip(svc, req_body),
    fields(
        product.id = %id,
        request_id = Empty,
    )
)]
pub async fn update_product(
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    Path(id): Path<Uuid>,
    Json(req_body): Json<UpdateProductReq>,
) -> ApiResult<JsonBody<ProductDto>> {
    let product = svc.product.update_product(id, req_body.into()).await?;
    Ok(Json(ProductDto::from(product)))
}

/// Delete a product by ID.
#[tracing::instrument(
    skip(svc),
    fields(
        product.id = %id,
        request_id = Empty,
    )
)]
pub async fn delete_product(
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    svc.product.delete_product(id).await?;
    Ok(no_content().into_response())
}
