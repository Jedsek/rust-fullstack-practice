// #![allow(unused)]

use backend::{
    db::{self, DbPool},
    handler,
};
use std::convert::Infallible;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = db::new_db().await.expect("Unable to create db pool");

    let pet = warp::path!("owner" / i32 / "pet");
    let pet_param = warp::path!("owner" / i32 / "pet" / i32);
    let owner = warp::path!("owner");

    let pet_routes = {
        let pet_get = warp::any()
            .and(with_db(db_pool.clone()))
            .and(pet)
            .and_then(handler::list_pets);

        let pet_post = warp::post()
            .and(with_db(db_pool.clone()))
            .and(pet)
            .and(warp::body::json())
            .and_then(handler::create_pet);

        let pet_delete = warp::delete()
            .and(with_db(db_pool.clone()))
            .and(pet_param)
            .and_then(handler::delete_pet);

        pet_get.or(pet_post).or(pet_delete)
    };

    // let owner_routes = {
    //     let owner_get = warp
    // }

    let routes = pet_routes;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

fn with_db(db_pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
