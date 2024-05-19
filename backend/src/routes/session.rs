use rocket::form::{Error, Errors, Form};
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

// TODO: The validation errors messages are not propagated to the 422 response
// TODO: Better form validation
#[derive(FromForm)]
struct CreateUser<'r> {
    #[field(validate = contains("@").or_else(msg!("Email address does not contain \"@\"")) )]
    email: &'r str,
    #[field(validate = omits("password").or_else(msg!("passwords can't contain the text \"password\"")) )]
    #[field(validate = len(8..).or_else(msg!("passwords must be at least 8 characters long")))]
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

#[post("/create")]
fn create_already_logged_in(_user: SessionUser) -> Status {
    Status::Forbidden
}

#[post("/create", data = "<create>", rank = 2)]
fn create(create: Form<CreateUser<'_>>) -> Status {
    // TODO: Actually create the user
    println!("Creating user with email: {}", create.email);
    Status::Ok
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        logout,
        who,
        who_no_login,
        create,
        create_already_logged_in
    ]
}
