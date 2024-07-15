use diesel::result::Error;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper,
};

use crate::db::DbConn;
use crate::models::locations::{Location, LocationResponse, NewLocation};

pub async fn get_bars(conn: &DbConn) -> Result<Vec<Location>, Error> {
    use crate::schema::locations::dsl::*;

    conn.run(|c| locations.filter(published.eq(true)).load(c))
        .await
}

pub async fn get_bars_with_visits(
    user_id: i32,
    conn: &DbConn,
) -> Result<Vec<LocationResponse>, Error> {
    use crate::schema::locations;
    use crate::schema::visits;

    conn.run(move |c| {
        locations::table
            .left_join(
                visits::table.on(visits::location_id
                    .eq(locations::id)
                    .and(visits::user_id.eq(user_id))),
            )
            .select((
                locations::id,
                locations::name,
                locations::description,
                locations::coordinates,
                locations::imageurl,
                locations::address_line,
                visits::visited_at.nullable(),
            ))
            .load(c)
    })
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
