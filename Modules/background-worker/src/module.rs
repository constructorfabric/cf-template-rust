use anyhow::Context;
use modkit::{Module, ModuleCtx, RunnableCapability, async_trait};
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use background_worker_sdk::PokemonClientV1;

use crate::config::{Config, default_interval};
use crate::domain::local_client::PokemonLocalClient;
use crate::domain::service::PokemonService;
use crate::infra::PokemonHttpRepository;

#[modkit::module(name = "background-worker", capabilities = [stateful])]
#[derive(Default)]
pub struct BackgroundWorkerModule {
    config: OnceLock<Config>,
    service: OnceLock<Arc<PokemonService>>,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[async_trait]
impl Module for BackgroundWorkerModule {
    async fn init(&self, ctx: &ModuleCtx) -> modkit::Result<()> {
        tracing::info!("Initializing background-worker module");
        self.config
            .set(ctx.config::<Config>()?)
            .map_err(|_| anyhow::anyhow!("config already initialized"))?;

        let repository = Arc::new(PokemonHttpRepository::new()?);
        let service = Arc::new(PokemonService::new(repository));
        let local_client = PokemonLocalClient::new(Arc::clone(&service));

        self.service
            .set(service)
            .map_err(|_| anyhow::anyhow!("service already initialized"))?;

        ctx.client_hub()
            .register::<dyn PokemonClientV1>(Arc::new(local_client));

        tracing::info!("background-worker registered PokemonClientV1 into ClientHub");

        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for BackgroundWorkerModule {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        tracing::info!("Starting background-worker background fetcher");

        let service = self
            .service
            .get()
            .context("service not initialized — was init() called?")?
            .clone();

        let interval_secs = self
            .config
            .get()
            .map(|c| c.interval)
            .unwrap_or_else(default_interval);

        let handle = tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_secs.get()));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        tracing::info!("background-worker fetcher cancelled");
                        break;
                    }
                    _ = interval.tick() => {
                        match service.fetch_random_pokemon().await {
                            Ok(pokemon) => {
                                tracing::debug!("Fetched pokemon: {pokemon:?}");
                            }
                            Err(e) => {
                                tracing::error!("Failed to fetch pokemon: {e}");
                            }
                        }
                    }
                }
            }
        });

        *self.task_handle.lock().await = Some(handle);

        Ok(())
    }

    async fn stop(&self, _cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        tracing::info!("Stopping background-worker module");

        if let Some(handle) = self.task_handle.lock().await.take() {
            if let Err(e) = handle.await {
                tracing::error!("background-worker task panicked: {e}");
            } else {
                tracing::info!("background-worker task completed gracefully");
            }
        } else {
            tracing::warn!("background-worker task was not running");
        }

        Ok(())
    }
}
