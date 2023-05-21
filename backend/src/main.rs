#[macro_use] extern crate rocket;
pub mod models;
pub mod schema;
pub mod db;

use models::Post;
use db::Db;
use diesel::prelude::*;
use rocket::serde::json::Json;


#[get("/")]
async fn index(connection: Db) -> Json<Vec<Post>> {
    use self::schema::posts::dsl::*;
    
    connection
        .run(|conn| posts.filter(published.eq(true)).limit(5).load(conn))
        .await
        .map(Json)
        .expect("Failed to fetch posts")
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::fairing()).mount("/", routes![index])
}
