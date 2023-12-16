use actix_web::{get, Responder, HttpResponse, post, patch, web::{Json, Path, Form, Data, self}, Scope, HttpRequest, http};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};

use crate::{middleware::authorization_token::AuthorizationToken, repository::{db::Db, user::Users}, models::user::User};

pub fn user_scope() -> Scope {
    web::scope("user")
        //.service(encode_token)
        .service(login)
        .service(register)
        .service(decode_token)
        .service(protected)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub email: String,
    pub exp: usize
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    id: usize,
    message: String
}

#[derive(Serialize, Deserialize)]
struct UserDataBody {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(body: web::Json<UserDataBody>, db: web::Data<Db>) -> impl Responder {

    println!("{} {}", body.email, body.password);


    HttpResponse::Ok().body("Your Authorized")
}

#[post("/register")]
async fn register(body: web::Json<UserDataBody>, usersRepo: web::Data<Users>) -> impl Responder {
    
    let users = usersRepo.get_users().await;

    usersRepo.create_user(User {
        id: 0,
        email: body.email.to_string(),
        password: body.password.to_string(),
        token: "".to_string()
    })
    .await;
    


    HttpResponse::Ok().body("user created")
}

//#[get("/encode-token/{id}")]
//async fn encode_token(path: web::Path<usize>) -> impl Responder {
//    let secret = String::from("secret");
//    let id: usize = path.into_inner();
//    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
//    let claims: Claims = Claims { id, exp };
//    let token: String = encode(
//        &Header::default(),
//        &claims,
//        &EncodingKey::from_secret(secret.as_str().as_ref())
//    ).unwrap();
//
//    HttpResponse::Ok().json(EncodeResponse {
//        message: "Your Token".to_string(),
//        token
//    })
//}

#[post("/decode-token")]
async fn decode_token(body: web::Json<DecodeBody>) -> impl Responder {
    let secret = String::from("secret");

    let decoded: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256)
    );

    match decoded {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
           message: "Authorized".to_string(),
           id: token.claims.id
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string()
        })
    }
}

#[get("/protected")]
async fn protected(auth_token: AuthorizationToken, req: HttpRequest) -> impl Responder {
    let token = req.headers().get(http::header::AUTHORIZATION).unwrap().to_str().unwrap();

    

    println!("{token}");
    
    HttpResponse::Ok().body("Your Authorized")
}
