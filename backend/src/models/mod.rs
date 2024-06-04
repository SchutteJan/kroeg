pub mod locations;
pub mod users;
pub mod visits;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}
