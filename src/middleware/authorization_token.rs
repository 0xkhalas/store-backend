use std::future::{Ready, ready};
use actix_web::{FromRequest, error::ErrorUnauthorized, http};
use serde::{ Serialize, Deserialize };

use crate::utils::jwt::decode_jwt;


#[derive(Serialize, Deserialize)]
pub struct AuthorizationToken {
    pub email: String,
    pub password: String,
    pub token: String
}

impl FromRequest for AuthorizationToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let secret = String::from("secret");

        // get auth
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        let auth_token = match auth_header {
            Some(s) => s.to_str().unwrap_or(""),
            None => ""
        };

        if auth_token.is_empty() { return ready(Err(ErrorUnauthorized("Invalid auth token!"))); }

        // decode token
        let decode = decode_jwt(auth_token.to_string(), secret);

        // return Token
        match decode {
            Some(o) => ready(Ok(o.data)),
            None => ready(Err(ErrorUnauthorized("Unauthorized!")))
        }
    }

}