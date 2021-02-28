mod database;
mod tests;

use database::TestDatabase;

use crate::service::{Service, Settings};

/// Test Suite to make testing the service easier.
pub struct TestSuite {
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
}
