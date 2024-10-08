use diesel::result::Error;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use postgis_diesel::functions::st_contains;

use crate::db::DbConn;
use crate::models::locations::{Location, LocationResponse, NewLocation, UpdateLocation};

pub async fn get_bars(only_published: bool, conn: &DbConn) -> Result<Vec<LocationResponse>, Error> {
    use crate::schema::areas;
    use crate::schema::locations::dsl::*;

    let mut query = locations
        .left_join(areas::table.on(st_contains(areas::area, coordinates)))
        .order_by(id.asc())
        .select((
            id,
            name,
            description,
            google_place_id,
            coordinates,
            published,
            imageurl,
            address_line,
            diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::Timestamp>>("NULL"),
            areas::name.nullable(),
            gem_ams_id,
        ))
        .into_boxed();

    if only_published {
        query = query.filter(published.eq(true))
    }
    conn.run(|c| query.load(c)).await
}

pub async fn get_bars_with_visits(
    user_id: i32,
    only_published: bool,
    conn: &DbConn,
) -> Result<Vec<LocationResponse>, Error> {
    use crate::schema::areas;
    use crate::schema::locations;
    use crate::schema::visits;

    let mut query = locations::table
        .left_join(
            visits::table.on(visits::location_id
                .eq(locations::id)
                .and(visits::user_id.eq(user_id))),
        )
        .left_join(areas::table.on(st_contains(areas::area, locations::coordinates)))
        .order_by(locations::id.asc())
        .select((
            locations::id,
            locations::name,
            locations::description,
            locations::google_place_id,
            locations::coordinates,
            locations::published,
            locations::imageurl,
            locations::address_line,
            visits::visited_at.nullable(),
            areas::name.nullable(),
            locations::gem_ams_id,
        ))
        .into_boxed();

    if only_published {
        query = query.filter(locations::published.eq(true));
    }

    conn.run(move |c| query.load(c)).await
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
