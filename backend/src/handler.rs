use crate::db::{self, DbPool};
// use anyhow::Result;
use common::{OwnerRequest, OwnerResponse, PetRequest, PetResponse};
use tap::prelude::*;
use warp::{http::StatusCode, reply, Reply};

type Result<T> = std::result::Result<T, warp::Rejection>;

pub async fn list_pets(db_pool: DbPool, owner_id: i32) -> Result<impl Reply> {
    let json = db::pet::fetch_all(&db_pool, owner_id)
        .await
        .expect("Unable to query db")
        .into_iter()
        .map(PetResponse::of)
        .collect::<Vec<_>>()
        .pipe_ref(reply::json);
    Ok(json)
}

pub async fn create_pet(db_pool: DbPool, owner_id: i32, body: PetRequest) -> Result<impl Reply> {
    let json = db::pet::create(&db_pool, owner_id, body)
        .await
        .expect("Unable to query db")
        .pipe(PetResponse::of)
        .pipe_ref(reply::json);
    Ok(json)
}

pub async fn delete_pet(db_pool: DbPool, owner_id: i32, id: i32) -> Result<impl Reply> {
    db::pet::delete(&db_pool, owner_id, id)
        .await
        .expect("Unable to query db");
    Ok(StatusCode::OK)
}

pub async fn list_owners(db_pool: DbPool) -> Result<impl Reply> {
    let json = db::owner::fetch_all(&db_pool)
        .await
        .expect("Unable to query db")
        .into_iter()
        .map(OwnerResponse::of)
        .collect::<Vec<_>>()
        .pipe_ref(reply::json);
    Ok(json)
}

pub async fn fetch_owner(db_pool: DbPool, id: i32) -> Result<impl Reply> {
    let json = db::owner::fetch_by_id(&db_pool, id)
        .await
        .expect("Unable to query db")
        .into_iter()
        .map(OwnerResponse::of)
        .collect::<Vec<_>>()
        .pipe_ref(reply::json);
    Ok(json)
}

pub async fn create_owner(db_pool: DbPool, body: OwnerRequest) -> Result<impl Reply> {
    let json = db::owner::create(&db_pool, body)
        .await
        .expect("Unable to query db")
        .pipe(OwnerResponse::of)
        .pipe_ref(reply::json);
    Ok(json)
}
