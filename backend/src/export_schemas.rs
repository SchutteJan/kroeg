use kroeg::models::locations::LocationResponse;
use kroeg::models::users::{Login, WhoResponse};
use rocket::serde::json::serde_json;
use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
struct ExportedSchemas {
    _location_response: LocationResponse,
    _user: (Login, WhoResponse),
}

fn main() {
    let schema = schema_for!(ExportedSchemas);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
