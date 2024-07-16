use kroeg::db::{areas, DbConn};
use kroeg::models::areas::Area;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/areas")]
async fn get_areas(conn: DbConn) -> Result<Json<Vec<Area>>, Status> {
    let areas = areas::get_areas(&conn).await;

    match areas {
        Ok(area_list) => Ok(Json(area_list)),
        Err(err) => {
            println!("Error: {:?}", err);
            Err(Status::InternalServerError)
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_areas]
}
