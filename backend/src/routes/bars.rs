use kroeg::db::{locations, DbConn};
use kroeg::models::locations::{Location, LocationResponse, NewLocation};
use kroeg::models::DeleteRequest;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::routes::AdminUser;

#[get("/bars")]
async fn bars(conn: DbConn) -> Result<Json<Vec<LocationResponse>>, Status> {
    let bars = locations::get_bars(&conn).await;

    match bars {
        Ok(bar_list) => Ok(Json(
            bar_list.iter().map(|l| LocationResponse::from(l)).collect(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/bar", data = "<bar>")]
async fn add_bar(
    conn: DbConn,
    bar: Json<NewLocation>,
    _user: AdminUser,
) -> Result<Json<LocationResponse>, Status> {
    let new_bar = NewLocation::from_json(&bar);

    let in_db = locations::add_bar(&conn, new_bar).await;

    match in_db {
        Ok(location) => Ok(Json(LocationResponse::from(&location))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/bar", data = "<bar>")]
async fn delete_bar(conn: DbConn, bar: Json<DeleteRequest>, _user: AdminUser) -> Status {
    let deleted_location = locations::delete_bar_by_id(&conn, bar.id).await;

    match deleted_location {
        Ok(0) => Status::NotFound,
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![add_bar, bars, delete_bar]
}
