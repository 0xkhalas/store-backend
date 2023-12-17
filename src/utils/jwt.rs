use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize)]
pub struct JwtData<T> {
    pub exp: usize,
    pub data: T,
}

pub fn create_jwt<T: Serialize>(object: &T, secret: String, days: i64) -> String {
    let exp: usize = (Utc::now() + Duration::days(days)).timestamp() as usize;

    let jwt_data = JwtData::<&T>{ exp, data: object };

    let token = encode(
        &Header::default(),
        &jwt_data,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    );

    token.unwrap_or("".to_string())
}

pub fn decode_jwt<T: DeserializeOwned>(token: String, secret: String) -> Option<JwtData<T>> {
    let decode: Result<TokenData<JwtData<T>>, jsonwebtoken::errors::Error> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256)
    );

    println!("{}", decode.is_ok());

    match decode {
        Ok(e) => Some(e.claims),
        Err(_) => None
    }
}