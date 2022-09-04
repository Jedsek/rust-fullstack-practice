pub mod owner;
pub mod pet;

use anyhow::Result;
use dotenvy::dotenv;
use sqlx::{pool::PoolOptions, postgres::Postgres, Pool};
use tokio::time::Duration;

pub type DbPool = Pool<Postgres>;

const MAX_CON: u32 = 32;
const TIMEOUT_CON: Duration = Duration::from_millis(500);
const TIMEOUT_IDLE: Duration = Duration::from_secs(3);

pub async fn new_db() -> Result<DbPool> {
    dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = PoolOptions::new()
        .max_connections(MAX_CON)
        .connect_timeout(TIMEOUT_CON)
        .idle_timeout(TIMEOUT_IDLE)
        .connect(&db_url)
        .await?;
    Ok(db_pool)
}
