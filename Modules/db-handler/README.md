# {{ project-name }}

Product management gear with REST API, database storage, and inter-gear communication via `ClientHub`.

## Gear Structure

```
{{ project-name }}/
├── sdk/                              # Standalone SDK crate for external consumers
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                    # Public exports
│       ├── models.rs                 # Product struct (transport-agnostic)
│       ├── errors.rs                 # ProductError (safe to expose externally)
│       ├── client.rs                 # ProductClientV1 trait + ProductStreamingClientV1
│       └── odata/
│           └── product.rs            # ProductQuery, ProductSchema, ProductFilterField
└── src/
    ├── lib.rs                        # Crate root and public re-exports
    ├── config.rs                     # ProductConfig (page sizes)
    ├── errors.rs                     # API error definitions
    ├── gear.rs                       # ProductGear — ToolKit registration point
    │
    ├── api/
    │   └── rest/
    │       ├── dto.rs                # ProductDto — REST wire format with serde/utoipa
    │       ├── error.rs              # DomainError → RFC 9457 Problem mapping
    │       ├── handlers/
    │       │   └── product.rs        # CRUD handler functions
    │       └── routes/
    │           └── product.rs        # OperationBuilder route + OpenAPI registration
    │
    ├── domain/
    │   ├── error.rs                  # DomainError + conversions to ProductError
    │   ├── repos/
    │   │   └── product_repo.rs       # ProductRepository trait (CRUD + list_page)
    │   ├── service/
    │   │   ├── mod.rs                # AppServices container, ServiceConfig, DbProvider
    │   │   └── product.rs            # ProductService — business logic
    │   └── local_client/
    │       ├── client.rs             # ProductLocalClient — implements ProductClientV1
    │       └── streaming.rs          # LocalProductStreamingClient — streaming adapter
    │
    └── infra/
        └── storage/
            ├── db.rs                 # db_err helper
            ├── entity/
            │   └── product.rs        # SeaORM entity (DeriveEntityModel, Scopable)
            ├── mapper.rs             # entity::Model → {{ crate_name }}_sdk::Product conversion
            ├── odata_mapper.rs       # ProductFilterField → SeaORM Column mapping
            ├── product_sea_repo.rs   # OrmProductRepository — implements ProductRepository
            └── migrations/
                └── m20260111_000001_initial.rs  # CREATE TABLE product
```

### Layer responsibilities

| Layer              | Package path                                | Rule                                                                     |
|--------------------|---------------------------------------------|--------------------------------------------------------------------------|
| **SDK**            | `{{ project-name }}-sdk` (`{{ crate_name }}_sdk`) | Public contract. No server code, no DB. Safe to expose to other gears. |
| **API**            | `crate::api`                                | HTTP concerns only. Translates HTTP ↔ domain. No business logic.         |
| **Domain**         | `crate::domain`                             | Business logic and rules. Must not import `api::*` or `infra::*`.        |
| **Infrastructure** | `crate::infra`                              | Database persistence. Implements domain repository traits.               |

---

## Data Flow

### GET /{{ project-name }}/v1/products/{id}

```
HTTP Request
    │
    ▼
axum::Router  (routes/product.rs)
    │  Extracts: Path(id), OData($select)
    │
    ▼
handlers::get_product  (handlers/product.rs)
    │  Calls: svc.product.get_product(&ctx, id)
    │
    ▼
ProductService::get_product  (domain/service/product.rs)
    │  Acquires: db.conn()  →  SecureConn
    │  Builds:   AccessScope::allow_all()
    │  Calls:    repo.get(&conn, &scope, id)
    │
    ▼
OrmProductRepository::get  (infra/storage/product_sea_repo.rs)
    │  Executes: ProductEntity::find()
    │              .filter(Column::Id = id)
    │              .secure().scope_with(scope)
    │              .one(conn)
    │
    ▼
mapper::From<entity::Model>  (infra/storage/mapper.rs)
    │  Converts: SeaORM Model → {{ crate_name }}_sdk::Product
    │
    ▼
handlers::get_product  (back in handler)
    │  Maps:    Product → ProductDto
    │  Applies: $select field projection (apply_select)
    │
    ▼
JSON Response  (axum::Json<serde_json::Value>)
```

### GET /{{ project-name }}/v1/products (paginated list)

```
HTTP Request  (?$filter=name eq 'Example'&$orderby=id desc&limit=20&cursor=...)
    │
    ▼
axum::Router  (routes/product.rs)
    │  Extracts: OData(query)
    │
    ▼
handlers::list_product  (handlers/product.rs)
    │  Calls: svc.product.list_product_page(&ctx, &query)
    │
    ▼
ProductService::list_product_page  (domain/service/product.rs)
    │  Acquires: db.conn()  →  SecureConn
    │  Builds:   AccessScope::allow_all()
    │  Calls:    repo.list_page(&conn, &scope, query)
    │
    ▼
OrmProductRepository::list_page  (infra/storage/product_sea_repo.rs)
    │  Calls: paginate_odata::<ProductFilterField, ProductODataMapper, ...>(
    │             base_query, conn, query, ("id", Desc), limit_cfg, Into::into
    │         )
    │
    ├── odata_mapper::ProductODataMapper   ($filter → SeaORM Condition)
    │     ProductFilterField::Id       → Column::Id
    │     ProductFilterField::Name     → Column::Name
    │     ProductFilterField::CreatedAt → Column::CreatedAt
    │
    └── mapper::From<entity::Model>   (per row: SeaORM Model → {{ crate_name }}_sdk::Product)
    │
    ▼
handlers::list_product  (back in handler)
    │  Maps:    Page<Product> → Page<ProductDto>
    │  Applies: $select field projection (page_to_projected_json)
    │
    ▼
JSON Response  (axum::Json<serde_json::Value>)
```

### Inter-gear communication (ClientHub)

Other gears can consume this gear without HTTP by obtaining the client from `ClientHub`:

```
gear.rs: ProductGear::init()
    └── registers Arc<ProductLocalClient> as dyn ProductClientV1

Consumer gear:
    let client = hub.get::<dyn ProductClientV1>()?;
    let product = client.get_product(id).await?;
    // or stream all results:
    let stream = client.product().stream(query_builder);
```

```
ProductClientV1 CRUD methods  (sdk/src/client.rs)
    │
    ▼
ProductLocalClient  (domain/local_client/client.rs)
    │  Converts: DomainError → ProductError
    │
    ▼
ProductService  (same domain service used by REST handlers)
```
