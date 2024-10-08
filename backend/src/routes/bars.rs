use diesel::result::Error;
use kroeg::db::{locations, DbConn};
use kroeg::img_proxy::get_proxied_image_url;
use kroeg::models::config::Config;
use kroeg::models::locations::{LocationResponse, NewLocation, UpdateLocation};
use kroeg::models::DeleteRequest;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::routes::{AdminUser, BasicUser};

fn replace_image_url_with_proxy(
    bars: Vec<LocationResponse>,
    config: &Config,
) -> Vec<LocationResponse> {
    bars.into_iter()
        .map(|l| {
            let proxied_image_url = match &l.imageurl {
                Some(image_url) => get_proxied_image_url(image_url, 300, 300, config).ok(),
                None => None,
            };
            LocationResponse {
                imageurl: proxied_image_url,
                ..l
            }
        })
        .collect()
}

#[get("/bars?<only_published>", rank = 2)]
async fn bars(
    conn: DbConn,
    only_published: Option<bool>,
    config: &State<Config>,
) -> Result<Json<Vec<LocationResponse>>, Status> {
    let bars = locations::get_bars(only_published.unwrap_or(true), &conn).await;

    match bars {
        Ok(bar_list) => Ok(Json(replace_image_url_with_proxy(bar_list, config))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/bars?<only_published>")]
async fn visited_bars(
    user: BasicUser,
    only_published: Option<bool>,
    config: &State<Config>,
    conn: DbConn,
) -> Result<Json<Vec<LocationResponse>>, Status> {
    let bars = locations::get_bars_with_visits(user.0, only_published.unwrap_or(true), &conn).await;

    match bars {
        Ok(bar_list) => Ok(Json(replace_image_url_with_proxy(bar_list, config))),
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

#[patch("/bar/<id>", data = "<bar>")]
async fn patch_bar(conn: DbConn, id: i32, bar: Json<UpdateLocation>, _user: AdminUser) -> Status {
    let updated = locations::update_bar_by_id(&conn, id, bar.into_inner()).await;

    match updated {
        Ok(0) => Status::NotFound,
        Ok(_) => Status::Ok,
        Err(err) => match err {
            Error::NotFound => Status::NotFound,
            Error::QueryBuilderError(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        },
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![add_bar, bars, delete_bar, visited_bars, patch_bar]
}
