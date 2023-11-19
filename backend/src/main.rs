extern crate diesel;
extern crate kroeg;
#[macro_use]
extern crate rocket;
mod routes;

use kroeg::db::Db;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::{Request, Response};
use serde::Deserialize;
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    // TODO: Remove before go-live
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    static_file_path: String,
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");

    println!("Serving static files from '{}'", config.static_file_path);

    rocket
        .attach(Db::fairing())
        .attach(CORS)
        .mount("/", routes::bars::routes())
        .mount("/", FileServer::from(config.static_file_path))
        .mount("/session", routes::session::routes())
}
