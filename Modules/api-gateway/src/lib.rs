#![doc = include_str!("../README.md")]

// ── Gear definition (aggregate root) ────────────────────────────────────────
mod gear;
pub use gear::{{ project-name | pascal_case }};

// ── Internal crate modules ───────────────────────────────────────────────────
mod config;
pub mod middleware;
mod web;

// ── Public re-exports ────────────────────────────────────────────────────────
pub use config::{{ project-name | pascal_case }}Config;
