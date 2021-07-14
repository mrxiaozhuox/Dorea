//! Dorea Storage Databasepub mod server;
#[allow(dead_code)]
use once_cell::sync::Lazy;

// Dorea db version (current)
const DOREA_VERSION: &'static str = "0.3.0";

// current version support load-storage version list.
#[allow(dead_code)]
const COMPATIBLE_VERSION: Lazy<Vec<String>> = Lazy::new(|| {
    vec![format!(
        "{:x}",
        md5::compute(format!("Dorea::{}", DOREA_VERSION).as_bytes())
    )]
});

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "processor")]
pub mod value;

#[cfg(feature = "processor")]
pub mod network;

mod command;
mod configure;
mod database;
mod handle;
mod logger;

type Result<T> = std::result::Result<T, anyhow::Error>;
