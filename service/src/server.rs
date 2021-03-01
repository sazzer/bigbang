pub(super) mod component;
mod span;

use std::sync::Arc;

use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use prometheus::Registry;

/// Trait implemented by all components that can contribute to the Actix service.
pub trait Configurer: Send + Sync {
    /// Configure some details onto the Actix service.
    ///
    /// # Parameters
    /// - `config` - The Actix `ServiceConfig` that routes and data can be wired onto.
    fn configure_server(&self, config: &mut ServiceConfig);
}

/// Wrapper around the HTTP Server.
pub struct Server {
    port: u16,
    pub(super) config: Vec<Arc<dyn Configurer>>,
    prometheus: Registry,
}

impl Server {
    /// Create a new instance of the HTTP Server.
    pub fn new(port: u16, config: Vec<Arc<dyn Configurer>>, prometheus: Registry) -> Self {
        tracing::debug!("Building HTTP Server");
        tracing::debug!("Built HTTP Server");

        Self {
            port,
            config,
            prometheus,
        }
    }

    /// Start the HTTP Server processing requests.
    pub async fn start(self) {
        tracing::debug!("Starting HTTP Server");

        let address = format!("0.0.0.0:{}", self.port);
        tracing::info!(address = ?address, "Starting HTTP server");

        let config = self.config.clone();
        let prometheus =
            PrometheusMetrics::new_with_registry(self.prometheus, "actix", Some("/metrics"), None)
                .unwrap();

        HttpServer::new(move || {
            let config = config.clone();
            let prometheus = prometheus.clone();

            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(prometheus)
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
                )
                .wrap(span::Span);

            for c in &config {
                app = app.configure(move |server_config| {
                    c.configure_server(server_config);
                });
            }

            tracing::trace!("Built listener");

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
