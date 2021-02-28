use deadpool_postgres::Pool;

pub(super) mod component;

/// Wrapper around a database connection pool
pub struct Database {
    pool: Pool,
}
