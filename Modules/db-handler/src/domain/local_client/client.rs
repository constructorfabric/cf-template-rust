use std::sync::Arc;

use toolkit::async_trait;
use toolkit_macros::domain_model;
use toolkit_odata::{ODataQuery, Page};
use uuid::Uuid;

#[cfg(feature = "odata")]
use {{ crate_name }}_sdk::ProductStreamingClientV1;
use {{ crate_name }}_sdk::{
    NewProduct, Product, ProductClientV1, ProductError, UpdateProductRequest,
};

#[cfg(feature = "odata")]
use crate::domain::local_client::streaming::LocalProductStreamingClient;
use crate::gear::ConcreteAppServices;

/// Local implementation of the object-safe `ProductClientV1`.
#[domain_model]
#[derive(Clone)]
pub struct ProductLocalClient {
    services: Arc<ConcreteAppServices>,
}

impl ProductLocalClient {
    #[must_use]
    pub(crate) fn new(services: Arc<ConcreteAppServices>) -> Self {
        Self { services }
    }
}

#[async_trait]
impl ProductClientV1 for ProductLocalClient {
    #[cfg(feature = "odata")]
    fn product(&self) -> Box<dyn ProductStreamingClientV1> {
        Box::new(LocalProductStreamingClient::new(Arc::clone(&self.services)))
    }

    async fn get_product(&self, id: Uuid) -> Result<Product, ProductError> {
        self.services
            .product
            .get_product(id)
            .await
            .map_err(ProductError::from)
    }

    async fn list_product(&self, query: ODataQuery) -> Result<Page<Product>, ProductError> {
        self.services
            .product
            .list_product_page(&query)
            .await
            .map_err(ProductError::from)
    }

    async fn create_product(&self, new_product: NewProduct) -> Result<Product, ProductError> {
        self.services
            .product
            .create_product(new_product)
            .await
            .map_err(ProductError::from)
    }

    async fn update_product(&self, req: UpdateProductRequest) -> Result<Product, ProductError> {
        self.services
            .product
            .update_product(req.id, req.patch)
            .await
            .map_err(ProductError::from)
    }

    async fn delete_product(&self, id: Uuid) -> Result<(), ProductError> {
        self.services
            .product
            .delete_product(id)
            .await
            .map_err(ProductError::from)
    }
}
