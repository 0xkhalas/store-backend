use actix_web::{get, Responder, HttpResponse, web, Scope};

use crate::repository::products::Products;


pub fn product_scope() -> Scope {
    web::scope("product")
        .service(protected)
}


#[get("/")]
async fn protected(_products_repo: web::Data<Products>) -> impl Responder {
    //products_repo.create_product().await;


    HttpResponse::Ok().body("Your Authorized")
}
