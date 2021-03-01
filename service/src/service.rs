#[cfg(test)]
pub mod testing;

use crate::server::Server;
use prometheus::Registry;

/// The actual service
pub struct Service {
    server: Server,
}

#[derive(Debug)]
pub struct Settings {
    pub port: u16,
    pub database_url: String,
}

impl Service {
    /// Create a new instance of the service
    #[tracing::instrument(name = "Service::new")]
    pub async fn new(settings: Settings) -> Self {
        tracing::debug!("Building Big Bang");

        let prometheus = Registry::new();
        let _db =
            crate::database::component::Component::new(&settings.database_url, &prometheus).await;

        let server = crate::server::component::Component::new(prometheus, settings.port);

        tracing::debug!("Built Big Bang");

        Self {
            server: server.server,
        }
    }

    /// Start the service processing requests
    pub async fn start(self) {
        tracing::info!("Starting Big Bang");
        self.server.start().await;
    }
}
