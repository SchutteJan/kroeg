use rocket::serde::Serialize;

pub mod bars;
pub mod session;

#[derive(Serialize)]
pub struct SessionUser(usize); // Is this the best location?
