use backend::db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = db::new_db().await?;
    db::owner::fetch_all(&db_pool).await?;

    Ok(())
}
