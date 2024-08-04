use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub static_file_path: String,
    pub img_proxy_domain: String,
    pub img_proxy_key: String,
    pub img_proxy_salt: String,
}
