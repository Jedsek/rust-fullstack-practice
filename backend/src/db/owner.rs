use crate::db::DbPool;
use anyhow::Result;
use common::{Owner, OwnerRequest};

pub async fn fetch_all(db_pool: &DbPool) -> Result<Vec<Owner>> {
    let owners = sqlx::query_as!(Owner, "SELECT * FROM owner")
        .fetch_all(db_pool)
        .await?;
    Ok(owners)
}

pub async fn fetch_one(db_pool: &DbPool, id: i32) -> Result<Vec<Owner>> {
    let owner = sqlx::query_as!(Owner, "SELECT * FROM owner WHERE id = $1", id)
        .fetch_all(db_pool)
        .await?;
    Ok(owner)
}

pub async fn create(db_pool: &DbPool, body: OwnerRequest) -> Result<Owner> {
    let owner = sqlx::query_as!(
        Owner,
        "INSERT INTO owner (name)VALUES($1) RETURNING *",
        body.name
    )
    .fetch_one(db_pool)
    .await?;
    Ok(owner)
}
