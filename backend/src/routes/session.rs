use diesel::prelude::*;
use kroeg::db::Db;
use kroeg::pgcrypto::{crypt, gen_salt};
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

// TODO: The validation errors messages are not propagated to the 422 response
// TODO: Better form validation
#[derive(FromForm)]
struct CreateUser {
    #[field(validate = contains("@").or_else(msg!("Email address does not contain \"@\"")) )]
    email: String,
    #[field(validate = omits("password").or_else(msg!("passwords can't contain the text \"password\"")) )]
    #[field(validate = len(8..).or_else(msg!("passwords must be at least 8 characters long")))]
    password: String,
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

async fn get_user_id_by_email(query_email: String, conn: &Db) -> Option<i32> {
    use kroeg::schema::users::dsl::*;

    let user_id = conn
        .run(move |c| {
            users
                .filter(email.eq(query_email))
                .select(id)
                .first::<i32>(c)
        })
        .await;

    user_id.ok()
}

async fn create_user(user: CreateUser, conn: &Db) -> Result<i32, diesel::result::Error> {
    use kroeg::schema::users::dsl::*;
    let user_id = conn
        .run(move |c| {
            diesel::insert_into(users)
                .values((
                    email.eq(user.email.clone()),
                    password.eq(crypt(user.password.clone(), gen_salt("bf"))),
                ))
                .returning(id)
                .get_result(c)
        })
        .await;

    user_id
}

#[post("/create", data = "<user>", rank = 2)]
async fn create(user: Form<CreateUser>, conn: Db) -> Status {
    let user_id = get_user_id_by_email(user.email.clone(), &conn).await;
    if user_id.is_some() {
        return Status::Conflict;
    }

    let _user_id = create_user(user.into_inner(), &conn).await.unwrap();

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
