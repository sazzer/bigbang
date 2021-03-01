mod database;
mod tests;

use actix_http::Request;
use database::TestDatabase;

use crate::service::{testing::TestResponse, Service, Settings};

/// Test Suite to make testing the service easier.
pub struct TestSuite {
    #[allow(dead_code)]
    database: TestDatabase,
    service: Service,
}

impl TestSuite {
    /// Create a new test suite.
    pub async fn new() -> Self {
        let _ = env_logger::try_init();

        let database = TestDatabase::new();
        let service = Service::new(Settings {
            port: 0,
            database_url: database.url.clone(),
        })
        .await;

        Self { database, service }
    }

    /// Inject a request into the service and get the response.
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
