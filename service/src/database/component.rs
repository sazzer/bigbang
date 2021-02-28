use std::{str::FromStr, sync::Arc};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};

use super::Database;

/// Component to represent the database connection
pub struct Component {
    pub database: Arc<Database>,
}

impl Component {
    /// Create a new database component.
    #[tracing::instrument(name = "Database::Component::new", skip())]
    pub async fn new(url: &str) -> Self {
        tracing::debug!("Building database connection");
        let pg_config = tokio_postgres::Config::from_str(url).expect("Invalid database URL");

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        pool.get()
            .await
            .expect("Unable to open database connection");

        tracing::debug!("Built database connection");

        let db = Database { pool };

        super::migrate::migrate(&db).await;

        Self {
            database: Arc::new(db),
        }
    }
}
