use serde::Deserialize;
use std::num::NonZeroU64;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_interval")]
    pub interval: NonZeroU64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interval: default_interval(),
        }
    }
}

#[allow(clippy::unwrap_used)] // The number is always non-zero
pub const DEFAULT_INTERVAL: NonZeroU64 = NonZeroU64::new(2).unwrap();

pub const fn default_interval() -> NonZeroU64 {
    DEFAULT_INTERVAL
}
