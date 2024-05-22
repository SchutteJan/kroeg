use kroeg::db::sql_types::UserRoleEnum;
use kroeg::db::{users, DbConn};
use kroeg::models::users::{Login, Register, WhoResponse};
use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;

use crate::routes::{AdminUser, BasicUser};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicUser {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<BasicUser, Self::Error> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(BasicUser)
            .or_forward(Status::Unauthorized)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<AdminUser, Self::Error> {
        request
            .cookies()
            .get_private("user_role")
            .and_then(|cookie| match cookie.value() {
                role if role == UserRoleEnum::Admin.to_string() => request
                    .cookies()
                    .get_private("user_id")
                    .and_then(|cookie| cookie.value().parse().ok())
                    .map(AdminUser),
                _ => None,
            })
            .or_forward(Status::Unauthorized)
    }
}

#[post("/login", data = "<login>")]
async fn login(jar: &CookieJar<'_>, login: Form<Login>, conn: DbConn) -> Status {
    match users::check_login_credentials(login.into_inner(), &conn).await {
        Ok((user_id, user_role)) => {
            jar.add_private(("user_id", user_id.to_string()));
            jar.add_private(("user_role", user_role.to_string()));
            Status::Ok
        }
        Err(e) => match e {
            diesel::result::Error::NotFound => Status::Unauthorized,
            _ => Status::InternalServerError,
        },
    }
}

#[get("/who")]
fn who_admin(user: AdminUser) -> Json<WhoResponse> {
    Json(WhoResponse {
        id: user.0,
        role: UserRoleEnum::Admin,
    })
}
#[get("/who", rank = 2)]
fn who(user: BasicUser) -> Json<WhoResponse> {
    Json(WhoResponse {
        id: user.0,
        role: UserRoleEnum::User,
    })
}

#[get("/who", rank = 3)]
fn who_no_login() -> Status {
    Status::Unauthorized
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> String {
    jar.remove_private("user_id");
    jar.remove_private("user_role");
    String::from("Logged out")
}

#[post("/create")]
fn create_already_logged_in(_user: BasicUser) -> Status {
    Status::Forbidden
}

#[post("/create", data = "<user>", rank = 2)]
async fn create(user: Form<Register>, conn: DbConn) -> Status {
    let user = user.into_inner();
    let email = user.email.as_ref().clone();
    let user_id = users::get_user_id_by_email(email, &conn).await;
    if user_id.is_some() {
        return Status::Conflict;
    }

    let _user_id = users::create_user(user, &conn).await.unwrap();

    Status::Created
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        logout,
        who,
        who_admin,
        who_no_login,
        create,
        create_already_logged_in
    ]
}
