use rocket::serde::Serialize;

pub mod areas;
pub mod bars;
pub mod session;
pub mod visits;

// Is this the best location for these types??
#[derive(Serialize)]
pub struct BasicUser(i32);
#[derive(Serialize)]
pub struct AdminUser(i32);
