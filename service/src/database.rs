use deadpool::managed::Object;
use deadpool_postgres::{ClientWrapper, Pool};
use postgres_types::ToSql;
use tokio_postgres::{IsolationLevel, Row};

pub(super) mod component;
mod migrate;

/// Wrapper around a database connection pool
pub struct Database {
    pool: Pool,
}

/// Wrapper around a connection to the database
pub struct Connection(Object<ClientWrapper, tokio_postgres::Error>);

/// Wrapper around a database transaction
pub struct Transaction<'a>(Option<deadpool_postgres::Transaction<'a>>);

impl Database {
    /// Get a new connection to the database from the connection pool
    pub async fn connect(&self) -> Connection {
        tracing::debug!("Getting database connection");
        let conn = self
            .pool
            .get()
            .await
            .expect("Failed to get database connection");

        Connection(conn)
    }
}

impl Connection {
    /// Begin a database transaction
    pub async fn begin(&mut self) -> Transaction<'_> {
        tracing::debug!("Starting transaction");

        let transaction = self
            .0
            .build_transaction()
            .isolation_level(IsolationLevel::Serializable)
            .read_only(false)
            .deferrable(false)
            .start()
            .await
            .expect("Failed to start transaction");

        Transaction(Some(transaction))
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        tracing::debug!("Returning database connection");
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        tracing::debug!("Finishing transaction");

        if self.0.is_some() {
            tracing::warn!("Transaction was not committed and will be rolled back");
        }
    }
}

impl<'a> Transaction<'a> {
    /// Execute a SQL statement within the transaction
    ///
    /// # Parameters
    /// - `sql` - The SQL statement to execute
    /// - `params` - Any bind parameters for the SQL statement
    ///
    /// # Returns
    /// The number of rows that were modified in the database
    pub async fn execute<S>(
        &self,
        sql: S,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error>
    where
        S: Into<String>,
    {
        let sql = sql.into();

        let span = tracing::trace_span!(
            "database::Transaction::execute",
            sql = sql.as_str(),
            result = tracing::field::Empty,
            error = tracing::field::Empty,
        );
        let _enter = span.enter();

        let tx = self.0.as_ref().unwrap();
        let result = tx.execute(sql.as_str(), params).await;

        if let Ok(r) = result {
            span.record("result", &r);
            span.record("error", &false);
        } else {
            span.record("error", &true);
        }

        result
    }

    /// Execute a SQL script within the transaction.
    /// Note that because this is considered to be an entire script and not just one statement, bind parameters are not available
    ///
    /// # Parameters
    /// - `sql` - The SQL statement to execute
    pub async fn batch_execute<S>(&self, sql: S) -> Result<(), tokio_postgres::Error>
    where
        S: Into<String>,
    {
        let sql = sql.into();

        let span = tracing::trace_span!(
            "database::Transaction::batch_execute",
            sql = sql.as_str(),
            error = tracing::field::Empty,
        );
        let _enter = span.enter();

        let tx = self.0.as_ref().unwrap();
        let result = tx.batch_execute(sql.as_str()).await;

        span.record("error", &result.is_err());

        result
    }

    /// Perform a SQL query within the transaction
    ///
    /// # Parameters
    /// - `sql` - The SQL query to perform
    /// - `params` - Any bind parameters for the SQL query
    ///
    /// # Returns
    /// The rows that were returned from the database
    pub async fn query<S>(
        &self,
        sql: S,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error>
    where
        S: Into<String>,
    {
        let sql = sql.into();

        let span = tracing::trace_span!(
            "database::Transaction::query",
            sql = sql.as_str(),
            rows = tracing::field::Empty,
            error = tracing::field::Empty,
        );
        let _enter = span.enter();

        let tx = self.0.as_ref().unwrap();
        let result = tx.query(sql.as_str(), params).await;

        if let Ok(r) = &result {
            span.record("rows", &r.len());
            span.record("error", &false);
        } else {
            span.record("error", &true);
        }

        result
    }

    /// Commit the transaction.
    /// This consumes the transaction object, after which it is not usable.
    pub async fn commit(mut self) -> Result<(), tokio_postgres::Error> {
        let span = tracing::trace_span!(
            "database::Transaction::commit",
            error = tracing::field::Empty,
        );
        let _enter = span.enter();

        let tx = self.0.take().unwrap();
        let result = tx.commit().await;

        span.record("error", &result.is_err());

        result
    }
}
