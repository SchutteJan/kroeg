use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;

use crate::routes::SessionUser;

#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionUser {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<SessionUser, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(SessionUser)
            .or_forward(Status::Unauthorized)
    }
}

#[post("/login", data = "<login>")]
fn login(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Status {
    // TODO Authentication backend
    if login.username == "jan" && login.password == "asd" {
        jar.add_private(("user_id", "999"));
        Status::Ok
    } else {
        Status::NotFound
    }
}

#[get("/who")]
fn who(user: SessionUser) -> Json<SessionUser> {
    Json(user)
}

#[get("/who", rank = 2)]
fn who_no_login() -> Status {
    Status::Unauthorized
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> String {
    jar.remove_private("user_id");
    String::from("Logged out")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, logout, who, who_no_login]
}
