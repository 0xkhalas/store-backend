use std::future::{Ready, ready};
use actix_web::{FromRequest, http::{header::HeaderValue, self}, error::ErrorUnauthorized, test::ok_service, Error};
use jsonwebtoken::{TokenData, DecodingKey, decode, Algorithm, Validation};
use serde::{ Serialize, Deserialize };


#[derive(Serialize, Deserialize)]
pub struct AuthorizationToken {
    pub id: usize
}

impl FromRequest for AuthorizationToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let secret = String::from("secret");

        // get auth
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        let auth_token = match auth_header {
            Some(s) => s.to_str().unwrap_or(""),
            None => ""
        };

        if auth_token.is_empty() { return ready(Err(ErrorUnauthorized("Invalid auth token!"))); }

        // decode token
        let decoded: Result<TokenData<AuthorizationToken>, jsonwebtoken::errors::Error> = decode(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256)
        );

        // return Token
        match decoded {
            Ok(token) => ready(Ok(AuthorizationToken {id: token.claims.id})),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized!")))
        }
    }

}