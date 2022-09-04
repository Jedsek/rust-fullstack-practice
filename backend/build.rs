use anyhow::Result;
use dotenvy::dotenv;
use sqlx::{migrate::MigrateDatabase, Connection, Executor, PgConnection};

#[tokio::main]
async fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/db/init_db.sql");
    println!("cargo:rerun-if-changed=.env");

    dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL")?;
    let db_url = db_url.as_str();

    if !sqlx::Postgres::database_exists(db_url).await? {
        sqlx::Postgres::create_database(db_url).await?;
    }

    let mut con = PgConnection::connect(db_url).await?;
    con.execute(include_str!("src/db/init_db.sql")).await?;

    Ok(())
}
