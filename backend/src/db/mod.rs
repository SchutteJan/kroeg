use rocket_sync_db_pools::database;

pub mod pgcrypto;
pub mod sql_types;

#[database("webapp")]
pub struct DbConn(diesel::PgConnection);
