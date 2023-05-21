use rocket_sync_db_pools::database;

#[database("diesel_demo")]
pub struct Db(diesel::PgConnection);
