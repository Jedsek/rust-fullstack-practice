use super::DbPool;
use anyhow::Result;
use common::{Pet, PetRequest};

pub async fn fetch_all(db_pool: &DbPool, owner_id: i32) -> Result<Vec<Pet>> {
    let pets = sqlx::query_as!(Pet, "SELECT * FROM pet WHERE owner_id = $1", owner_id)
        .fetch_all(db_pool)
        .await?;
    Ok(pets)
}

pub async fn create(db_pool: &DbPool, owner_id: i32, body: PetRequest) -> Result<Pet> {
    let pet = sqlx::query_as!(
        Pet,
        "INSERT INTO pet (name, owner_id, animal_type, color)VALUES($1,$2,$3,$4) RETURNING *",
        body.name,
        owner_id,
        body.animal_type,
        body.color
    )
    .fetch_one(db_pool)
    .await?;
    Ok(pet)
}

pub async fn delete(db_pool: &DbPool, owner_id: i32, id: i32) -> Result<()> {
    sqlx::query!(
        "DELETE FROM pet WHERE owner_id = $1 AND id = $2",
        owner_id,
        id
    )
    .execute(db_pool)
    .await?;
    Ok(())
}
