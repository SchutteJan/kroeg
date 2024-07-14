use std::fmt;
use std::io::Write;

use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::*;
use schemars::JsonSchema;
use serde::Serialize;

/*
    The file implements the UserRole enum SqlType for Diesel. Enum in postgres are custom types,
    therefore we need to provide a full SqlType implementation for every enum we use. Luckily the
    Diesel source code has an example implementation which this file is based on:
    https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
*/
#[derive(SqlType)]
#[diesel(postgres_type(name = "UserRole"))]
pub struct UserRole;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, JsonSchema)]
#[diesel(sql_type = UserRole)]
pub enum UserRoleEnum {
    Admin,
    User,
}

impl fmt::Display for UserRoleEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ToSql<UserRole, Pg> for UserRoleEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserRoleEnum::Admin => out.write_all(b"admin")?,
            UserRoleEnum::User => out.write_all(b"user")?,
        }
        Ok(IsNull::No)
    }
}

// TODO: read https://docs.rs/diesel/latest/diesel/deserialize/trait.FromSql.html
impl FromSql<UserRole, Pg> for UserRoleEnum {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(UserRoleEnum::Admin),
            b"user" => Ok(UserRoleEnum::User),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(SqlType)]
#[diesel(postgres_type(name = "AreaType"))]
pub struct AreaType;
