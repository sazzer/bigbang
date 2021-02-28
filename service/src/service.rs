use crate::server::Server;

/// The actual service
pub struct Service {
    server: Server,
}

impl Service {
    /// Create a new instance of the service
    #[tracing::instrument(name = "Service::new")]
    pub async fn new() -> Self {
        tracing::debug!("Building Big Bang");

        let server = Server::new();

        tracing::debug!("Built Big Bang");

        Self { server }
    }

    /// Start the service processing requests
    pub async fn start(&self) {
        tracing::info!("Starting Big Bang");
        self.server.start().await;
    }
}
