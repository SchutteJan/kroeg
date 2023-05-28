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


use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;


#[get("/")]
async fn index() -> String {
    "Try /bars".to_string()
}


#[get("/bars")]
async fn bars(conn: Db) -> Json<Vec<Location>> {
    use schema::locations::dsl::*;

    let result = conn.run(|c| 
        locations.filter(published.eq(true)).load(c)
    );
    result.await.map(Json).expect("A result")
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(CORS)
        .mount("/", routes![index, bars])
}
