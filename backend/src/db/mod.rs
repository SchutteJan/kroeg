use rocket_sync_db_pools::database;

pub mod locations;
pub mod pgcrypto;
pub mod sql_types;
pub mod users;

#[database("webapp")]
pub struct DbConn(diesel::PgConnection);
