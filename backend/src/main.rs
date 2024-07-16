#[macro_use]
extern crate rocket;

mod routes;

use std::path::Path;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use kroeg::db::DbConn;
use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, NamedFile};
use rocket::{Build, Rocket, State};
use serde::Deserialize;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

async fn run_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = DbConn::get_one(&rocket).await.expect("database connection");
    db.run(|conn| match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}

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
        .attach(AdHoc::try_on_ignite("Database Migrations", run_migrations))
        .mount("/", routes::bars::routes())
        .mount("/", routes::areas::routes())
        .mount("/", FileServer::from(config.static_file_path))
        .mount("/session", routes::session::routes())
        .mount("/visit", routes::visits::routes())
        .mount("/", routes![catch_all])
}
