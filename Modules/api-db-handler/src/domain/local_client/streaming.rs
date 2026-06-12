use std::pin::Pin;
use std::sync::Arc;

use {{ crate_name }}_sdk::odata::PokemonSchema;
use {{ crate_name }}_sdk::{Pokemon, PokemonError, PokemonStreamingClientV1};
use futures_util::{Stream, StreamExt};
use toolkit_macros::domain_model;
use toolkit_sdk::odata::{QueryBuilder, items_stream_boxed};
use toolkit_sdk::pager::PagerError;

use crate::module::ConcreteAppServices;

#[domain_model]
pub(crate) struct LocalPokemonStreamingClient {
    services: Arc<ConcreteAppServices>,
}

impl LocalPokemonStreamingClient {
    #[must_use]
    pub fn new(services: Arc<ConcreteAppServices>) -> Self {
        Self { services }
    }
}

impl PokemonStreamingClientV1 for LocalPokemonStreamingClient {
    fn stream(
        &self,
        query: QueryBuilder<PokemonSchema>,
    ) -> Pin<Box<dyn Stream<Item = Result<Pokemon, PokemonError>> + Send + 'static>> {
        let services = Arc::clone(&self.services);
        let stream = items_stream_boxed(
            query,
            Box::new(move |q| {
                let services = Arc::clone(&services);
                Box::pin(async move {
                    services
                        .pokemon
                        .list_pokemon_page(&q)
                        .await
                        .map_err(PokemonError::from)
                })
            }),
        );
        Box::pin(stream.map(|res| {
            res.map_err(|err| match err {
                PagerError::Fetch(pokemon_err) => pokemon_err,
                PagerError::InvalidCursor(_) => PokemonError::streaming(err.to_string()),
            })
        }))
    }
}
