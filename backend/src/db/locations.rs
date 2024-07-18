use diesel::result::Error;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use postgis_diesel::functions::st_contains;

use crate::db::DbConn;
use crate::models::locations::{Location, LocationResponse, NewLocation, UpdateLocation};

pub async fn get_bars(conn: &DbConn) -> Result<Vec<LocationResponse>, Error> {
    use crate::schema::areas;
    use crate::schema::locations::dsl::*;

    conn.run(|c| {
        locations
            .filter(published.eq(true))
            .left_join(areas::table.on(st_contains(areas::area, coordinates)))
            .select((
                id,
                name,
                description,
                coordinates,
                imageurl,
                address_line,
                diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::Timestamp>>(
                    "NULL",
                ),
                areas::name.nullable(),
            ))
            .load(c)
    })
    .await
}

pub async fn get_bars_with_visits(
    user_id: i32,
    conn: &DbConn,
) -> Result<Vec<LocationResponse>, Error> {
    use crate::schema::areas;
    use crate::schema::locations;
    use crate::schema::visits;

    conn.run(move |c| {
        locations::table
            .left_join(
                visits::table.on(visits::location_id
                    .eq(locations::id)
                    .and(visits::user_id.eq(user_id))),
            )
            .left_join(areas::table.on(st_contains(areas::area, locations::coordinates)))
            .select((
                locations::id,
                locations::name,
                locations::description,
                locations::coordinates,
                locations::imageurl,
                locations::address_line,
                visits::visited_at.nullable(),
                areas::name.nullable(),
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

pub async fn update_bar_by_id(
    conn: &DbConn,
    bar_id: i32,
    bar: UpdateLocation,
) -> Result<usize, Error> {
    use crate::schema::locations;
    use crate::schema::locations::id;

    conn.run(move |c| {
        diesel::update(locations::table.filter(id.eq(bar_id)))
            .set(&bar)
            .execute(c)
    })
    .await
}
