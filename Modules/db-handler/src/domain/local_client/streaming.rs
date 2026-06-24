use std::pin::Pin;
use std::sync::Arc;

use {{ crate_name }}_sdk::odata::ProductSchema;
use {{ crate_name }}_sdk::{Product, ProductError, ProductStreamingClientV1};
use futures_util::{Stream, StreamExt};
use toolkit_macros::domain_model;
use toolkit_sdk::odata::{QueryBuilder, items_stream_boxed};
use toolkit_sdk::pager::PagerError;

use crate::gear::ConcreteAppServices;

#[domain_model]
pub(crate) struct LocalProductStreamingClient {
    services: Arc<ConcreteAppServices>,
}

impl LocalProductStreamingClient {
    #[must_use]
    pub fn new(services: Arc<ConcreteAppServices>) -> Self {
        Self { services }
    }
}

impl ProductStreamingClientV1 for LocalProductStreamingClient {
    fn stream(
        &self,
        query: QueryBuilder<ProductSchema>,
    ) -> Pin<Box<dyn Stream<Item = Result<Product, ProductError>> + Send + 'static>> {
        let services = Arc::clone(&self.services);
        let stream = items_stream_boxed(
            query,
            Box::new(move |q| {
                let services = Arc::clone(&services);
                Box::pin(async move {
                    services
                        .product
                        .list_product_page(&q)
                        .await
                        .map_err(ProductError::from)
                })
            }),
        );
        Box::pin(stream.map(|res| {
            res.map_err(|err| match err {
                PagerError::Fetch(product_err) => product_err,
                PagerError::InvalidCursor(_) => ProductError::streaming(err.to_string()),
            })
        }))
    }
}
