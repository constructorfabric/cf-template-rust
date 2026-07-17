use crate::config::{Config, default_interval};
use toolkit::{Gear, GearCtx, RunnableCapability, async_trait};
use std::sync::OnceLock;

#[derive(Default)]
#[toolkit::gear(name = "hello-world", capabilities = [stateful])]
pub struct HelloWorldGear {
    config: OnceLock<Config>,
}

#[async_trait]
impl Gear for HelloWorldGear {
    async fn init(&self, ctx: &GearCtx) -> toolkit::Result<()> {
        tracing::info!("Init hello world gear");
        self.config
            .set(ctx.config::<Config>()?)
            .map_err(|_| anyhow::anyhow!("config already initialized"))?;
        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for HelloWorldGear {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> toolkit::Result<()> {
        let interval_secs = self
            .config
            .get()
            .map_or_else(default_interval, |c| c.interval);

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

    async fn stop(&self, _cancel: tokio_util::sync::CancellationToken) -> toolkit::Result<()> {
        tracing::info!("Goodbye World");
        Ok(())
    }
}
