use diesel::prelude::*;
use diesel::Selectable;
use rocket::form::{DataField, FromFormField, ValueField};
use rocket::serde::{Deserialize, Serialize};
use rocket::{form, FromForm};
use schemars::JsonSchema;

use crate::db::sql_types::UserRoleEnum;
use crate::schema::users;

#[derive(Serialize, JsonSchema)]
pub struct WhoResponse {
    pub id: i32,
    pub role: UserRoleEnum,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(FromForm, JsonSchema)]
pub struct Login {
    pub email: Email,
    pub password: String,
}

// TODO: The validation errors messages are not propagated to the 422 response
// TODO: Better form validation
#[derive(FromForm, JsonSchema)]
pub struct Register {
    pub email: Email,
    #[field(
        validate = omits("password").or_else(msg ! ("passwords can't contain the text \"password\""))
    )]
    #[field(validate = len(8..).or_else(msg ! ("passwords must be at least 8 characters long")))]
    pub password: String,
}

// This wrapped type is to allow a custom FromFormField implementation that lowercases the email
#[derive(JsonSchema)]
pub struct Email(String);

impl Email {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<String> for Email {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

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
