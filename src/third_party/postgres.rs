use once_cell::sync::OnceCell;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Pool, Postgres};
use std::io::Result;

pub(crate) struct ConnectionPool {
    pool: Pool<Postgres>,
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self {
            pool: default_pool(),
        }
    }
}

#[tokio::main]
async fn default_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@127.0.0.1/postgres")
        .await
        .unwrap()
}

impl ConnectionPool {
    pub(crate) fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub(crate) async fn init_db_pool() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@127.0.0.1/postgres")
        .await
        .unwrap();
    DB_POOL.set(pool.clone()).unwrap();
    Ok(pool)
}

pub(crate) async fn get_db_conn() -> Pool<Postgres> {
    DB_POOL.get().unwrap().clone()
}
