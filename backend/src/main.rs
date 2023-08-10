#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate kroeg;

use kroeg::db::Db;
use kroeg::models::{Location, LocationResponse, NewLocation};

use diesel::result::Error;
use diesel::SelectableHelper;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use rocket::serde::json::Json;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::{Request, Response};
use serde::Deserialize;

pub struct CORS;

async fn get_bars(conn: Db) -> Result<Vec<Location>, Error> {
    use kroeg::schema::locations::dsl::*;

    conn.run(|c| locations.filter(published.eq(true)).load(c))
        .await
}

#[get("/bars")]
async fn bars(conn: Db) -> Json<Vec<LocationResponse>> {
    let bars: Vec<Location> = get_bars(conn).await.expect("has values");

    let response = bars.iter().map(|l| LocationResponse::from(l)).collect();

    Json(response)
}

#[post("/bar", data = "<bar>")]
async fn add_bar(conn: Db, bar: Json<NewLocation>) -> Json<LocationResponse> {
    // TODO: Find a better way of processing all of these structures
    use kroeg::models::Point;
    use kroeg::schema::locations;

    let coordinate = Point {
        x: bar.coordinates.x,
        y: bar.coordinates.y,
        srid: Some(4326),
    };
    let new_bar = NewLocation {
        name: bar.name.clone(),
        coordinates: coordinate,
        published: bar.published,
        description: bar.description.clone(),
        osm_node_id: bar.osm_node_id.clone(),
        google_place_id: bar.google_place_id.clone(),
    };

    let in_db = conn
        .run(|conn| {
            diesel::insert_into(locations::table)
                .values(new_bar)
                .returning(Location::as_returning())
                .get_result(conn)
        })
        .await;

    Json(LocationResponse::from(&in_db.expect("Inserted")))
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
        .mount("/", routes![bars, add_bar])
        .mount("/", FileServer::from(config.static_file_path))
}
