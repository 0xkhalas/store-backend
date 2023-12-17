use sqlx::prelude::FromRow;

#[derive(FromRow, Debug)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub categorie: String,
    pub price: i32,
    pub count: i32,
    pub image: String,
}