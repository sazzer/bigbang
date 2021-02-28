mod span;

use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use prometheus::Registry;

/// Wrapper around the HTTP Server.
pub struct Server {
    port: u16,
    prometheus: Registry,
}

impl Server {
    /// Create a new instance of the HTTP Server.
    #[tracing::instrument(name = "Server::new")]
    pub fn new(prometheus: Registry) -> Self {
        tracing::debug!("Building HTTP Server");
        tracing::debug!("Built HTTP Server");

        Self {
            port: 8000,
            prometheus,
        }
    }

    /// Start the HTTP Server processing requests.
    pub async fn start(self) {
        tracing::debug!("Starting HTTP Server");

        let address = format!("0.0.0.0:{}", self.port);
        tracing::info!(address = ?address, "Starting HTTP server");

        let prometheus =
            PrometheusMetrics::new_with_registry(self.prometheus, "actix", Some("/metrics"), None)
                .unwrap();

        HttpServer::new(move || {
            let prometheus = prometheus.clone();

            let app = App::new()
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
