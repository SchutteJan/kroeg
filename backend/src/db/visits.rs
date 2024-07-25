use diesel::dsl::count_distinct;
use diesel::result::Error;
use diesel::{BoolExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};
use futures::try_join;
use postgis_diesel::functions::st_contains;

use crate::db::DbConn;
use crate::models::visits::{NewVisit, Visit, VisitStats};

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

pub async fn get_user_visit_stats(
    current_user_id: i32,
    conn: &DbConn,
) -> Result<VisitStats, Error> {
    use diesel::{ExpressionMethods, RunQueryDsl};

    use crate::schema::areas;
    use crate::schema::locations;
    use crate::schema::visits;

    let distinct_bar_visits_fut = conn.run(move |c| {
        visits::table
            .inner_join(locations::table)
            .select(count_distinct(visits::location_id))
            .filter(visits::user_id.eq(current_user_id))
            .filter(locations::published.eq(true))
            .first::<i64>(c)
    });

    let bar_visits_by_area_fut = conn.run(move |c| {
        locations::table
            .filter(locations::published.eq(true))
            .inner_join(areas::table.on(st_contains(areas::area, locations::coordinates)))
            .inner_join(
                visits::table.on(visits::location_id
                    .eq(locations::id)
                    .and(visits::user_id.eq(current_user_id))),
            )
            .group_by(areas::name)
            .select((areas::name, count_distinct(locations::id)))
            .order_by(areas::name)
            .load::<(String, i64)>(c)
    });

    let total_bars_by_area_fut = conn.run(move |c| {
        locations::table
            .inner_join(areas::table.on(st_contains(areas::area, locations::coordinates)))
            .filter(locations::published.eq(true))
            .group_by(areas::name)
            .select((areas::name, count_distinct(locations::id)))
            .order_by(areas::name)
            .load::<(String, i64)>(c)
    });

    match try_join!(
        distinct_bar_visits_fut,
        total_bars_by_area_fut,
        bar_visits_by_area_fut
    ) {
        Ok((distinct_bar_visits, total_bars_by_area, bar_visits_by_area)) => Ok(VisitStats {
            distinct_bar_visits,
            total_bars_by_area,
            bar_visits_by_area,
        }),
        Err(e) => Err(e),
    }
}
