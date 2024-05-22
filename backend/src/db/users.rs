use crate::db::sql_types::UserRoleEnum;
use crate::db::DbConn;
use crate::models::users::{CreateUser, Login};

pub async fn get_user_id_by_email(query_email: String, conn: &DbConn) -> Option<i32> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    use crate::schema::users::dsl::*;

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

pub async fn create_user(user: CreateUser, conn: &DbConn) -> Result<i32, diesel::result::Error> {
    use diesel::{ExpressionMethods, RunQueryDsl};

    use crate::db::pgcrypto::{crypt, gen_salt};
    use crate::schema::users::dsl::*;
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

pub async fn check_login_credentials(
    login: Login,
    conn: &DbConn,
) -> Result<(i32, UserRoleEnum), diesel::result::Error> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    use crate::db::pgcrypto::crypt;
    use crate::schema::users::dsl::*;

    let user_id_role = conn
        .run(move |c| {
            users
                .filter(email.eq(login.email.into_inner()))
                .filter(password.eq(crypt(login.password, password)))
                .select((id, role))
                .first::<(i32, UserRoleEnum)>(c)
        })
        .await;
    user_id_role
}
