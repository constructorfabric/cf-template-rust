use anyhow::Context;
use toolkit::{Gear, GearCtx, RunnableCapability, async_trait};
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use {{ crate_name }}_sdk::PokemonClientV1;

use crate::config::{Config, default_interval};
use crate::domain::local_client::PokemonLocalClient;
use crate::domain::service::PokemonService;
use crate::infra::PokemonHttpRepository;

#[toolkit::gear(name = "{{ project-name }}", capabilities = [stateful])]
#[derive(Default)]
pub struct {{ crate_name | pascal_case }}Module {
    config: OnceLock<Config>,
    service: OnceLock<Arc<PokemonService>>,
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[async_trait]
impl Gear for {{ crate_name | pascal_case }}Module {
    async fn init(&self, ctx: &GearCtx) -> toolkit::Result<()> {
        tracing::info!("Initializing {{ project-name }} module");
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

        tracing::info!("{{ project-name }} registered PokemonClientV1 into ClientHub");

        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for {{ crate_name | pascal_case }}Module {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> toolkit::Result<()> {
        tracing::info!("Starting {{ project-name }} background fetcher");

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
                        tracing::info!("{{ project-name }} fetcher cancelled");
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

    async fn stop(&self, _cancel: tokio_util::sync::CancellationToken) -> toolkit::Result<()> {
        tracing::info!("Stopping {{ project-name }} module");

        if let Some(handle) = self.task_handle.lock().await.take() {
            if let Err(e) = handle.await {
                tracing::error!("{{ project-name }} task panicked: {e}");
            } else {
                tracing::info!("{{ project-name }} task completed gracefully");
            }
        } else {
            tracing::warn!("{{ project-name }} task was not running");
        }

        Ok(())
    }
}
