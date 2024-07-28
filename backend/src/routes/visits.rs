use kroeg::db::visits::delete_visit;
use kroeg::db::{visits, DbConn};
use kroeg::models::visits::NewVisit;
use rocket::http::Status;

use crate::routes::BasicUser;

#[post("/bar/<bar_id>")]
async fn add_bar_visit(conn: DbConn, bar_id: i32, user: BasicUser) -> Status {
    let new_visit = NewVisit {
        user_id: user.0,
        location_id: bar_id,
        visited_at: chrono::Utc::now().naive_utc(),
    };

    let in_db = visits::add_visit(new_visit, &conn).await;

    match in_db {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[delete("/bar/<bar_id>")]
async fn delete_bar_visit(conn: DbConn, bar_id: i32, user: BasicUser) -> Status {
    match delete_visit(user.0, bar_id, &conn).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![add_bar_visit, delete_bar_visit]
}
