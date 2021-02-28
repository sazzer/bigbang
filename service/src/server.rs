use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, App, HttpServer};

/// Wrapper around the HTTP Server.
pub struct Server {
    port: u16,
}

impl Server {
    /// Create a new instance of the HTTP Server.
    #[tracing::instrument(name = "Server::new")]
    pub fn new() -> Self {
        tracing::debug!("Building HTTP Server");
        tracing::debug!("Built HTTP Server");

        Self { port: 8000 }
    }

    /// Start the HTTP Server processing requests.
    pub async fn start(&self) {
        tracing::debug!("Starting HTTP Server");

        let address = format!("0.0.0.0:{}", self.port);
        tracing::info!(address = ?address, "Starting HTTP server");

        HttpServer::new(move || {
            let app = App::new().wrap(Logger::default()).wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
            );

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
