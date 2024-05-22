use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::DbConn;
use crate::models::locations::{Location, NewLocation};

pub async fn get_bars(conn: &DbConn) -> Result<Vec<Location>, Error> {
    use crate::schema::locations::dsl::*;

    conn.run(|c| locations.filter(published.eq(true)).load(c))
        .await
}

pub async fn add_bar(conn: &DbConn, bar: NewLocation) -> Result<Location, Error> {
    use crate::schema::locations;

    conn.run(|c| {
        diesel::insert_into(locations::table)
            .values(bar)
            .returning(Location::as_returning())
            .get_result(c)
    })
    .await
}

pub async fn delete_bar_by_id(conn: &DbConn, bar_id: i32) -> Result<usize, Error> {
    use crate::schema::locations;
    use crate::schema::locations::id;
    conn.run(move |c| {
        diesel::delete(locations::table)
            .filter(id.eq(bar_id))
            .execute(c)
    })
    .await
}
