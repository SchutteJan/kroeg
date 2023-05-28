#[macro_use]
extern crate rocket;
extern crate diesel;

pub mod models;
pub mod schema;
pub mod db;


use models::Location;
use db::Db;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;


#[get("/")]
async fn index(conn: Db) -> Json<Vec<Location>> {
    use schema::locations::dsl::*;

    let result = conn.run(|c| 
        locations.filter(published.eq(true)).load(c)
    );
    result.await.map(Json).expect("A result")
}



#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::fairing()).mount("/", routes![index])
}
