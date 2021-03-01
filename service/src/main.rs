#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod database;
mod home;
#[cfg(test)]
mod integration;
mod server;
mod service;

use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

/// Representation of the application settings that will be loaded from the environment
#[derive(Debug, Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: u16,
    /// The connection URL for the database
    pub database_url: String,
}

impl Default for Settings {
    /// Construct the settings from the environment
    ///
    /// # Returns
    /// The Settings object, loaded from the environment variables
    fn default() -> Self {
        let mut s = Config::new();
        s.set_default("port", 8000)
            .expect("Failed to set default value for 'port'");

        s.merge(Environment::default())
            .expect("Failed to load environment properties");

        s.try_into().expect("Failed to build settings from config")
    }
}

impl From<Settings> for service::Settings {
    fn from(settings: Settings) -> Self {
        Self {
            port: settings.port,
            database_url: settings.database_url,
        }
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("bigbang")
        .from_env()
        .install()
        .unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let settings = Settings::default();
    tracing::debug!(settings = ?settings, "Loaded settings");

    let service = service::Service::new(settings.into()).await;
    service.start().await;
}
