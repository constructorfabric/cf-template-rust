//! Only compiled when the `odata` feature is enabled — the list route requires
//! typed OData filter fields from the SDK's `odata` module.
use super::{dto, handlers};
use {{ crate_name }}_sdk::odata::ProductFilterField;
use axum::Router;
use toolkit::api::OpenApiRegistry;
use toolkit::api::operation_builder::{OperationBuilder, OperationBuilderODataExt};

const API_TAG: &str = "Products";

pub(super) fn register_product_routes(mut router: Router, openapi: &dyn OpenApiRegistry) -> Router {
    // GET /{{ project-name }}/v1/products - List products with cursor-based pagination
    router = OperationBuilder::get("/{{ project-name }}/v1/products")
        .operation_id("{{ crate_name }}.products.list")
        .summary("List products with cursor pagination")
        .description("Retrieve a paginated list of products using cursor-based pagination")
        .tag(API_TAG)
        .public()
        .query_param_typed(
            "limit",
            false,
            "Maximum number of products to return",
            "integer",
        )
        .query_param("cursor", false, "Cursor for pagination")
        .handler(handlers::list_product)
        .json_response_with_schema::<toolkit_odata::Page<dto::ProductDto>>(
            openapi,
            http::StatusCode::OK,
            "Paginated list of products",
        )
        .with_odata_filter::<ProductFilterField>()
        .with_odata_select()
        .with_odata_orderby::<ProductFilterField>()
        .error_400(openapi)
        .error_500(openapi)
        .register(router, openapi);

    // GET /{{ project-name }}/v1/products/{id} - Get a specific product
    router = OperationBuilder::get("/{{ project-name }}/v1/products/{id}")
        .operation_id("{{ crate_name }}.products.get")
        .public()
        .summary("Get product by ID")
        .description("Retrieve a specific product by their UUID")
        .tag(API_TAG)
        .path_param("id", "Product UUID")
        .handler(handlers::get_product)
        .with_odata_select()
        .json_response_with_schema::<dto::ProductDto>(
            openapi,
            http::StatusCode::OK,
            "Product found",
        )
        .error_404(openapi)
        .error_500(openapi)
        .register(router, openapi);

    // POST /{{ project-name }}/v1/products - Create a new product
    router = OperationBuilder::post("/{{ project-name }}/v1/products")
        .operation_id("{{ crate_name }}.products.create")
        .public()
        .summary("Create product")
        .description("Create a product with the provided data")
        .tag(API_TAG)
        .json_request::<dto::CreateProductReq>(openapi, "Product creation data")
        .handler(handlers::create_product)
        .json_response_with_schema::<dto::ProductDto>(
            openapi,
            http::StatusCode::CREATED,
            "Created product",
        )
        .error_400(openapi)
        .error_409(openapi)
        .error_500(openapi)
        .register(router, openapi);

    // PATCH /{{ project-name }}/v1/products/{id} - Update a product
    router = OperationBuilder::patch("/{{ project-name }}/v1/products/{id}")
        .operation_id("{{ crate_name }}.products.update")
        .public()
        .summary("Update product")
        .description("Partially update a product with the provided fields")
        .tag(API_TAG)
        .path_param("id", "Product UUID")
        .json_request::<dto::UpdateProductReq>(openapi, "Product update data")
        .handler(handlers::update_product)
        .json_response_with_schema::<dto::ProductDto>(
            openapi,
            http::StatusCode::OK,
            "Updated product",
        )
        .error_400(openapi)
        .error_404(openapi)
        .error_500(openapi)
        .register(router, openapi);

    // DELETE /{{ project-name }}/v1/products/{id} - Delete a product
    router = OperationBuilder::delete("/{{ project-name }}/v1/products/{id}")
        .operation_id("{{ crate_name }}.products.delete")
        .public()
        .summary("Delete product")
        .description("Delete a product by UUID")
        .tag(API_TAG)
        .path_param("id", "Product UUID")
        .handler(handlers::delete_product)
        .json_response(http::StatusCode::NO_CONTENT, "Product deleted")
        .error_404(openapi)
        .error_500(openapi)
        .register(router, openapi);

    router
}
