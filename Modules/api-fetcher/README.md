# {{ project-name }}

Periodically fetches a random public API item from [JSONPlaceholder](https://jsonplaceholder.typicode.com/) and exposes the
fetch capability to other gears via `ClientHub`.

## Gear structure

```
{{ project-name }}/
├── sdk/                        # Public API crate ({{ project-name }}-sdk)
│   └── src/
│       ├── client.rs           # PublicApiItemClientV1 trait
│       ├── errors.rs           # PublicApiItemError (public)
│       └── models.rs           # PublicApiItem (public)
└── src/
    ├── domain/
    │   ├── error.rs            # DomainError (internal)
    │   ├── ports.rs            # PublicApiItemRepository trait (port)
    │   ├── service.rs          # PublicApiItemService (domain logic)
    │   └── local_client.rs     # PublicApiItemLocalClient (SDK adapter)
    ├── infra/
    │   ├── mod.rs              # PublicApiItemHttpRepository (HTTP impl of PublicApiItemRepository)
    │   └── model.rs            # PublicApiItemResponse (raw API shape)
    └── gear.rs                 # {{ crate_name | pascal_case }}Gear (toolkit wiring)
```

### Layer responsibilities

| Layer      | What it does                                                                                                                                                                          |
|------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **sdk**    | Defines the public contract (`PublicApiItemClientV1` trait, `PublicApiItem` model, `PublicApiItemError`). Other gears depend only on this crate.                                                        |
| **domain** | Pure business logic. `PublicApiItemService` drives the use-case through the `PublicApiItemRepository` port. Errors stay internal (`DomainError`).                                                 |
| **infra**  | `PublicApiItemHttpRepository` implements `PublicApiItemRepository` by calling the JSONPlaceholder over HTTP. Maps the raw `PublicApiItemResponse` to the SDK `PublicApiItem` type.                                    |
| **gear**   | Wires everything together. `init()` constructs the object graph and registers `PublicApiItemLocalClient` into `ClientHub`. `start()` runs the background polling loop via `PublicApiItemService`. |

### Data flow

```
ClientHub consumer
  └─▶ PublicApiItemClientV1 (sdk trait)
        └─▶ PublicApiItemLocalClient (domain/local_client.rs)
              └─▶ PublicApiItemService (domain/service.rs)
                    └─▶ PublicApiItemRepository port (domain/ports.rs)
                          └─▶ PublicApiItemHttpRepository (infra/mod.rs)
                                └─▶ JSONPlaceholder (HTTPS)
```

## Background polling

In addition to on-demand access via `ClientHub`, the gear spawns a background task (every 5 s)
that calls `PublicApiItemService::fetch_random_public_api_item()` and logs the result. The loop is cancelled
gracefully via a `CancellationToken` when the application shuts down.

## Dependencies

- `cf-gears-toolkit` — gear framework (`Gear`, `RunnableCapability`, `GearCtx`, `ClientHub`)
- `cf-gears-toolkit-http` — `HttpClient` wrapper
- `{{ project-name }}-sdk` — public SDK (path dependency)
- `anyhow` — error handling in toolkit boundaries
- `thiserror` — `DomainError` derive
- `async-trait` — object-safe async traits
- `tokio` / `tokio-util` — async runtime and cancellation
- `tracing` — structured logging
- `serde` — JSON deserialization of JSONPlaceholder responses
