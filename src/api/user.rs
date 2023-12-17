use actix_web::{get, Responder, HttpResponse, post, Scope, HttpRequest, http, web};
use serde::{Serialize, Deserialize};

use crate::{middleware::authorization_token::AuthorizationToken, repository, utils::jwt::create_jwt, models::user::User};

#[derive(Serialize, Deserialize)]
struct UserDataBody {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    message: String,
    token: String
}

#[derive(Serialize, Deserialize)]
struct MessageResponse {
    message: String,
}

pub fn user_scope() -> Scope {
    web::scope("user")
        //.service(encode_token)
        .service(login)
        .service(register)
        .service(protected)
}

#[post("/login")]
async fn login(body: web::Json<UserDataBody>, users_repo: web::Data<repository::users::Users>) -> impl Responder {
    let current_user = users_repo.get_user(&body.email)
        .await;

    match current_user {
        Some(e) => {
            if body.email == e.email && body.password == e.password {
                return HttpResponse::Ok().json(LoginResponse {
                    message: "Login Token".to_string(),
                    token: e.token
                });
            }

            HttpResponse::BadRequest().json(MessageResponse {
                message: "fityou".to_string()
            })
        }
        None => {
            HttpResponse::BadRequest().json(MessageResponse {
                message: "fityou".to_string()
            })
        }
    }
}

#[post("/register")]
async fn register(body: web::Json<UserDataBody>, users_repo: web::Data<repository::users::Users>) -> impl Responder {
    if users_repo.is_exists(&body.email).await {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: "You have One".to_string()
        });
    }

    let token = create_jwt::<UserDataBody>(&body.0, "secret".to_string(), 365);

    users_repo.create_user(User {
        email: body.email.to_string(),
        password: body.password.to_string(),
        token
    })
    .await;

    HttpResponse::Ok().json(MessageResponse {
        message: "user created".to_string()
    })
}

#[get("/protected")]
async fn protected(_auth_token: AuthorizationToken, req: HttpRequest) -> impl Responder {
    let token = req.headers().get(http::header::AUTHORIZATION).unwrap().to_str().unwrap();

    println!("{token}");
    
    HttpResponse::Ok().body("Your Authorized")
}

#[test]
fn asd() {
    let jwt = create_jwt::<UserDataBody>(&UserDataBody {
        email: "xxalir0@gmail.com".to_string(),
        password: "assdasd".to_string()
    }, "secret".to_string(), 365);

    let jwt_decode = decode_jwt::<UserDataBody>(jwt.to_string(), "secret".to_string()).unwrap();

    println!("{}", jwt);
}