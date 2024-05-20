use rocket::serde::Serialize;

pub mod bars;
pub mod session;

// Is this the best location for these types??
#[derive(Serialize)]
pub struct BasicUser(usize);
#[derive(Serialize)]
pub struct AdminUser(usize);
