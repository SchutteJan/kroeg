use rocket_sync_db_pools::database;

#[database("webapp")]
pub struct Db(diesel::PgConnection);
