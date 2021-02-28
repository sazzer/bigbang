/// The actual service
pub struct Service {}

impl Service {
    /// Create a new instance of the service
    #[tracing::instrument(name = "Service::new")]
    pub async fn new() -> Self {
        tracing::debug!("Building Big Bang");
        tracing::debug!("Built Big Bang");

        Self {}
    }

    /// Start the service processing requests
    pub async fn start(&self) {
        tracing::info!("Starting Big Bang");
    }
}
