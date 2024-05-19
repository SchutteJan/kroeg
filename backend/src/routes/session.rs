use diesel::prelude::*;
use kroeg::db::Db;
use kroeg::pgcrypto::{crypt, gen_salt};
use rocket::form;
use rocket::form::{DataField, Form, FromFormField, ValueField};
use rocket::http::{CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;

use crate::routes::SessionUser;

// This wrapped type is to allow a custom FromFormField implementation that lowercases the email
struct Email(String);

impl Email {
    fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<String> for Email {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

// TODO: Move this elsewhere
#[rocket::async_trait]
impl<'r> FromFormField<'r> for Email {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        // TODO: Better validation than checking the length and for the @ character
        if field.value.len() > 320 {
            return Err(form::Error::validation("Email address too long"))?;
        }

        match field.value.find('@') {
            Some(_) => Ok(Email(field.value.to_lowercase())),
            None => Err(form::Error::validation("does not contain '@'"))?,
        }
    }

    async fn from_data(_field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // An implementation was made in commit 427f6fea8c46, but I removed it because it didn't
        // "feel" safe and added the memchr crate to the dependencies.
        unimplemented!("from_data not implemented for Email")
    }
}

#[derive(FromForm)]
struct Login {
    email: Email,
    password: String,
}

// TODO: The validation errors messages are not propagated to the 422 response
// TODO: Better form validation
#[derive(FromForm)]
struct CreateUser {
    email: Email,
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
async fn login(jar: &CookieJar<'_>, login: Form<Login>, conn: Db) -> Status {
    let user_id = check_login_credentials(login.into_inner(), &conn).await;
    if user_id.is_none() {
        return Status::Unauthorized;
    }

    jar.add_private(("user_id", user_id.unwrap().to_string()));
    Status::Ok
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

async fn check_login_credentials(login: Login, conn: &Db) -> Option<i32> {
    use kroeg::schema::users::dsl::*;

    let user_id = conn
        .run(move |c| {
            users
                .filter(email.eq(login.email.into_inner()))
                .filter(password.eq(crypt(login.password, password)))
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
                    email.eq(user.email.into_inner().clone()),
                    password.eq(crypt(user.password.clone(), gen_salt("bf"))),
                ))
                .returning(id)
                .get_result(c)
        })
        .await;

    user_id
}

#[post("/create")]
fn create_already_logged_in(_user: SessionUser) -> Status {
    Status::Forbidden
}

#[post("/create", data = "<user>", rank = 2)]
async fn create(user: Form<CreateUser>, conn: Db) -> Status {
    let user = user.into_inner();
    let email = user.email.as_ref().clone();
    let user_id = get_user_id_by_email(email, &conn).await;
    if user_id.is_some() {
        return Status::Conflict;
    }

    let _user_id = create_user(user, &conn).await.unwrap();

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
