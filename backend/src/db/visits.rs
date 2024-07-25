use diesel::dsl::count_distinct;
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

    match try_join!(distinct_bar_visits_fut) {
        Ok((distinct_bar_visits,)) => Ok(VisitStats {
            distinct_bar_visits,
        }),
        Err(e) => Err(e),
    }
}
