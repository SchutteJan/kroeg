use diesel::sql_function;
use diesel::sql_types::Text;

// SQL function for the pgcrypto extension
// https://www.postgresql.org/docs/current/pgcrypto.html

sql_function! {
    // crypt(password text, salt text) returns text
    // https://www.postgresql.org/docs/current/pgcrypto.html#PGCRYPTO-PASSWORD-HASHING-FUNCS-CRYPT
    fn crypt(password: Text, salt: Text) -> Text;
}
sql_function! {
    // gen_salt(type text [, iter_count integer ]) returns text
    // type: Accepted types: des, xdes, md5 and bf.
    // iter_count has been omitted to ensure default values are used
    // https://www.postgresql.org/docs/current/pgcrypto.html#PGCRYPTO-PASSWORD-HASHING-FUNCS-GEN-SALT
    fn gen_salt(t: Text) -> Text;
}
