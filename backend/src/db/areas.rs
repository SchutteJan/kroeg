use diesel::result::Error;
use diesel::RunQueryDsl;

use crate::db::DbConn;
use crate::models::areas::Area;

pub async fn get_areas(conn: &DbConn) -> Result<Vec<Area>, Error> {
    use crate::schema::areas::dsl::*;
    conn.run(|c| areas.load(c)).await
}
