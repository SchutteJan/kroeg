use rocket_sync_db_pools::database;

#[database("webapp")]
pub struct DbConn(diesel::PgConnection);
