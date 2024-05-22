use kroeg::models::locations::LocationResponse;
use rocket::serde::json::serde_json;
use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
struct ExportedSchemas {
    _location_response: LocationResponse,
}

fn main() {
    let schema = schema_for!(ExportedSchemas);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
