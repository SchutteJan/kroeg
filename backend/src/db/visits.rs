use diesel::dsl::{count, count_distinct};
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use futures::try_join;

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

    use crate::schema::visits::dsl::*;

    let total_visits_fut = conn.run(move |c| {
        visits
            .select(count(id))
            .filter(user_id.eq(current_user_id))
            .first::<i64>(c)
    });

    let distinct_bar_visits_fut = conn.run(move |c| {
        visits
            .select(count_distinct(location_id))
            .filter(user_id.eq(current_user_id))
            .first::<i64>(c)
    });

    match try_join!(total_visits_fut, distinct_bar_visits_fut) {
        Ok((total_visits, distinct_bar_visits)) => Ok(VisitStats {
            distinct_bar_visits,
            total_bar_visits: total_visits,
        }),
        Err(e) => Err(e),
    }
}
