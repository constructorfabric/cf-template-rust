# {{ project-name }}-sdk

Public API crate for the `{{ project-name }}` gear. Depend on this crate to fetch PublicApiItem data
from any other gear in your ConstructorFabric gears application — without taking a direct dependency on the
gear itself.

## Types

| Type              | Description                              |
|-------------------|------------------------------------------|
| `PublicApiItemClientV1` | Async trait to obtain from `ClientHub`   |
| `PublicApiItem`         | Returned model (`id`, `name`, `title`)  |
| `PublicApiItemError`    | Error type returned by the trait methods |

## Usage

### 1. Add the dependency

```toml
# your-gear/Cargo.toml
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
`init()`), so make sure the gear is included in your application's gear list.

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

Your gear's `init()` runs after all gears have been loaded, so the client is always available
by the time any gear's `start()` or request handlers execute.
