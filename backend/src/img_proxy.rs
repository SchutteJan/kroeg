use base64::prelude::*;
use hex::{self, FromHexError};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use urlencoding::encode;

use crate::models::config::Config;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub enum Error {
    InvalidKey(FromHexError),
    InvalidSalt(FromHexError),
}

pub fn get_proxied_image_url(
    original_uri: &str,
    width: i32,
    height: i32,
    config: &Config,
) -> Result<String, Error> {
    let encoded_uri = encode(original_uri).into_owned();
    let img_proxy_path = format!("/rs:fit:{width}:{height}/plain/{encoded_uri}");

    let signed_path = sign_path(
        &img_proxy_path,
        config.img_proxy_key.as_str(),
        config.img_proxy_salt.as_str(),
    );
    match signed_path {
        Ok(signed_path) => Ok(format!("{}{}", config.img_proxy_domain, signed_path)),
        Err(err) => Err(err),
    }
}

pub fn sign_path(path: &str, key: &str, salt: &str) -> Result<String, Error> {
    let decoded_key = match hex::decode(key) {
        Ok(key) => key,
        Err(err) => return Err(Error::InvalidKey(err)),
    };
    let decoded_salt = match hex::decode(salt) {
        Ok(salt) => salt,
        Err(err) => return Err(Error::InvalidSalt(err)),
    };

    let mut hmac = HmacSha256::new_from_slice(&decoded_key).unwrap();
    hmac.update(&decoded_salt);
    hmac.update(path.as_bytes());

    let signature = hmac.finalize().into_bytes();
    let signature = BASE64_URL_SAFE_NO_PAD.encode(signature.as_slice());
    let signed_path = format!("/{}{}", signature, path);

    Ok(signed_path)
}
