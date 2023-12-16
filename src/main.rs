mod models;
mod api;
mod repository;
mod middleware;

use actix_web::{HttpServer, App, web::Data};
use api::user;
use repository::{db::Db, user::Users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Db::new().await;

        // Create Tables
    db.init_tables().await;


    let users_app_data = Data::new(Users::new(db));
    
    // Create Database
        // Create HttpServer
    HttpServer::new(move || {
        App::new()
            .app_data(users_app_data.clone())
            // pizza endpoint
            .service(api::user::user_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}