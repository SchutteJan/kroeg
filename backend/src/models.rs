use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}