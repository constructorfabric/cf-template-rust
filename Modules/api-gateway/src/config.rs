//! `{{ project-name | pascal_case }}Config` — runtime configuration for the API gateway module.
//!
//! Loaded by `ToolKit` from the host application's config file under the
//! `modules.{{ project-name }}.config` key.

use serde::{Deserialize, Serialize};

fn default_bind_addr() -> String {
    "127.0.0.1:8080".to_owned()
}

fn default_timeout_secs() -> u64 {
    30
}

/// Configuration for the API gateway module.
///
/// Example YAML:
/// ```yaml
/// modules:
///   {{ project-name }}:
///     config:
///       bind_addr: "0.0.0.0:8080"
///       timeout_secs: 30
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct {{ project-name | pascal_case }}Config {
    /// Socket address the HTTP server binds to (e.g. `"127.0.0.1:8080"`).
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// Per-request timeout in seconds. Requests that exceed this limit receive
    /// `408 Request Timeout`. Defaults to 30 s.
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

impl Default for {{ project-name | pascal_case }}Config {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            timeout_secs: default_timeout_secs(),
        }
    }
}
