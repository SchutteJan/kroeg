#[macro_use]
extern crate rocket;
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

use db::Db;
use models::{Location, LocationResponse};

use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use rocket::serde::json::Json;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

async fn get_bars(conn: Db) -> Result<Vec<Location>, Error> {
    use schema::locations::dsl::*;

    conn.run(|c| locations.filter(published.eq(true)).load(c))
        .await
}

#[get("/bars")]
async fn bars(conn: Db) -> Json<Vec<LocationResponse>> {
    let bars: Vec<Location> = get_bars(conn).await.expect("has values");

    let response = bars.iter().map(|l| LocationResponse::from(l)).collect();

    Json(response)
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .attach(CORS)
        .mount("/", routes![bars])
        .mount("/", FileServer::from("static"))
}
