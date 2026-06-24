# {{ project-name }}-sdk

Public API crate for the `{{ project-name }}` module. Depend on this crate to fetch PublicApiItem data
from any other module in your ConstructorFabric gears application — without taking a direct dependency on the
module itself.

## Types

| Type              | Description                              |
|-------------------|------------------------------------------|
| `PublicApiItemClientV1` | Async trait to obtain from `ClientHub`   |
| `PublicApiItem`         | Returned model (`id`, `name`, `title`)  |
| `PublicApiItemError`    | Error type returned by the trait methods |

## Usage

### 1. Add the dependency

```toml
# your-module/Cargo.toml
[dependencies]
{{ project-name }}-sdk = { path = "../{{ project-name }}/sdk" }
```

### 2. Obtain the client from `ClientHub`

```rust
use {{ crate_name }}_sdk::PublicApiItemClientV1;

async fn example(hub: &ClientHub) -> toolkit::Result<()> {
    let client = hub.get::<dyn PublicApiItemClientV1>()?;
    let public_api_item = client.fetch_random_public_api_item().await?;
    println!("{} — title {}", public_api_item.name, public_api_item.title);
    Ok(())
}
```

`hub.get` returns an error if `{{ project-name }}` was not loaded (i.e. not registered during
`init()`), so make sure the module is included in your application's module list.

### 3. Handle errors

```rust
use {{ crate_name }}_sdk::{PublicApiItemClientV1, PublicApiItemError};

match client.fetch_random_public_api_item().await {
    Ok(public_api_item) => println!("Got: {}", public_api_item.name),
    Err(PublicApiItemError::Internal(msg)) => eprintln!("fetch failed: {msg}"),
}
```

## How the client is registered

`{{ project-name }}` registers the implementation during its `init()` phase:

```rust
ctx.client_hub().register::<dyn PublicApiItemClientV1>(Arc::new(local_client));
```

Your module's `init()` runs after all modules have been loaded, so the client is always available
by the time any module's `start()` or request handlers execute.
