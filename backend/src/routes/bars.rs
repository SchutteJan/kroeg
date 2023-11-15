use diesel::result::Error;
use diesel::SelectableHelper;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use kroeg::db::Db;
use kroeg::models::{Location, LocationResponse, NewLocation};
use rocket::serde::json::Json;

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
        imageurl: bar.imageurl.clone(),
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

pub fn routes() -> Vec<rocket::Route> {
    routes![add_bar, bars]
}
