use crate::config::{Config, default_interval};
use modkit::{Module, ModuleCtx, RunnableCapability, async_trait};
use std::sync::OnceLock;

#[derive(Default)]
#[modkit::module(name = "hello-world", capabilities = [stateful])]
pub struct HelloWorldModule {
    config: OnceLock<Config>,
}

#[async_trait]
impl Module for HelloWorldModule {
    async fn init(&self, ctx: &ModuleCtx) -> modkit::Result<()> {
        tracing::info!("Init hello world module");
        self.config
            .set(ctx.config::<Config>()?)
            .map_err(|_| anyhow::anyhow!("config already initialized"))?;
        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for HelloWorldModule {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        let interval_secs = self
            .config
            .get()
            .map(|c| c.interval)
            .unwrap_or_else(default_interval);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    () = cancel.cancelled() => {
                        tracing::info!("Cancelled World");
                        break
                    },
                    () = tokio::time::sleep(tokio::time::Duration::from_secs(interval_secs.get())) => {
                        tracing::info!("Hello World");
                    }
                }
            }
        });
        Ok(())
    }

    async fn stop(&self, _cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        tracing::info!("Goodbye World");
        Ok(())
    }
}
