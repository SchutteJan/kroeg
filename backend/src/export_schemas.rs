extern crate rocket;

pub mod models;

use models::LocationResponse;
use rocket::serde::json::serde_json;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(LocationResponse);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
