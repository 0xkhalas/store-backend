mod models;
mod api;
mod repository;
mod middleware;
mod utils;

use actix_web::{HttpServer, App, web::Data};
use repository::{db::Db, users::Users, products::Products};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Db::new().await;

    // Create Tables
    db.init_tables().await;

    let users_app_data = Data::new(Users::new(db.clone()));
    let product_app_data = Data::new(Products::new(db.clone()));
    
    // Create Database
        // Create HttpServer
    HttpServer::new(move || {
        App::new()
            .app_data(users_app_data.clone())
            .app_data(product_app_data.clone())
            // scopes
            .service(api::user::user_scope())
            .service(api::product::product_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}