# {{ project-name }}-sdk

Public API crate for the `{{ project-name }}` module. Depend on this crate to fetch Pokemon data
from any other module in your ConstructorFabric gears application — without taking a direct dependency on the
module itself.

## Types

| Type              | Description                              |
|-------------------|------------------------------------------|
| `PokemonClientV1` | Async trait to obtain from `ClientHub`   |
| `Pokemon`         | Returned model (`id`, `name`, `height`)  |
| `PokemonError`    | Error type returned by the trait methods |

## Usage

### 1. Add the dependency

```toml
# your-module/Cargo.toml
[dependencies]
{{ project-name }}-sdk = { path = "../{{ project-name }}/sdk" }
```

### 2. Obtain the client from `ClientHub`

```rust
use {{ crate_name }}_sdk::PokemonClientV1;

async fn example(hub: &ClientHub) -> toolkit::Result<()> {
    let client = hub.get::<dyn PokemonClientV1>()?;
    let pokemon = client.fetch_random_pokemon().await?;
    println!("{} — height {}", pokemon.name, pokemon.height);
    Ok(())
}
```

`hub.get` returns an error if `{{ project-name }}` was not loaded (i.e. not registered during
`init()`), so make sure the module is included in your application's module list.

### 3. Handle errors

```rust
use {{ crate_name }}_sdk::{PokemonClientV1, PokemonError};

match client.fetch_random_pokemon().await {
    Ok(pokemon) => println!("Got: {}", pokemon.name),
    Err(PokemonError::Internal(msg)) => eprintln!("fetch failed: {msg}"),
}
```

## How the client is registered

`{{ project-name }}` registers the implementation during its `init()` phase:

```rust
ctx.client_hub().register::<dyn PokemonClientV1>(Arc::new(local_client));
```

Your module's `init()` runs after all modules have been loaded, so the client is always available
by the time any module's `start()` or request handlers execute.
