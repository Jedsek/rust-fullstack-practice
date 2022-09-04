use backend::{
    db::{self, DbPool},
    handler,
};
use warp::{
    hyper::{header::*, Method},
    Filter,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = db::new_db().await.expect("Unable to create db pool");

    let with_db = |db_pool: DbPool| warp::any().map(move || db_pool.clone());
    let pet_routes = {
        let pet = warp::path!("owner" / i32 / "pet");
        let pet_param = warp::path!("owner" / i32 / "pet" / i32);

        let pet_get = warp::get()
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

    let owner_routes = {
        let owner = warp::path!("owner");
        let owner_param = warp::path!("owner" / i32);

        let owner_get = warp::get()
            .and(with_db(db_pool.clone()))
            .and(owner)
            .and_then(handler::list_owners)
            .or(warp::get()
                .and(with_db(db_pool.clone()))
                .and(owner_param)
                .and_then(handler::fetch_owner_by_id));
        let owner_post = warp::post()
            .and(owner)
            .and(with_db(db_pool.clone()))
            .and(warp::body::json())
            .and_then(handler::create_owner);
        owner_get.or(owner_post)
    };

    let routes = pet_routes.or(owner_routes).with(warp::log("Routes")).with(
        warp::cors()
            .allow_any_origin()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
                Method::OPTIONS,
            ])
            .allow_credentials(true)
            .expose_headers([LINK])
            .allow_headers([CONTENT_TYPE, ACCEPT])
            .max_age(300),
    );
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}
