use diesel::result::Error;
use diesel::{RunQueryDsl, SelectableHelper};

use crate::db::DbConn;
use crate::models::visits::{NewVisit, Visit};

pub async fn add_visit(new_visit: NewVisit, conn: &DbConn) -> Result<Visit, Error> {
    use crate::schema::visits;

    conn.run(|c| {
        diesel::insert_into(visits::table)
            .values(new_visit)
            .returning(Visit::as_returning())
            .get_result(c)
    })
    .await
}
