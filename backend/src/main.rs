#[macro_use]
extern crate rocket;

mod routes;

use std::path::Path;

use kroeg::db::DbConn;
use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, NamedFile};
use rocket::State;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    static_file_path: String,
}

#[get("/<_..>", rank = 1000)]
async fn catch_all(config: &State<Config>) -> Option<NamedFile> {
    // SPAs should be rendered by static file servers with a fallback page
    // https://kit.svelte.dev/docs/single-page-apps#usage

    let path = Path::new(config.static_file_path.as_str()).join("200.html");
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");

    println!("Serving static files from '{}'", config.static_file_path);

    rocket
        .attach(DbConn::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes::bars::routes())
        .mount("/", FileServer::from(config.static_file_path))
        .mount("/session", routes::session::routes())
        .mount("/visit", routes::visits::routes())
        .mount("/", routes![catch_all])
}
