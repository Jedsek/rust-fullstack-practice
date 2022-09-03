use anyhow::Result;
use dotenvy::dotenv;
use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=./src/db/migrations/");

    dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL")?;
    let mut con = PgConnection::connect(&db_url).await?;
    sqlx::migrate!("./src/db/migrations").run(&mut con).await?;

    Ok(())
}
