use crate::models::product::Product;

use super::db::Db;

pub struct Products {
    pub db: Db
}

impl Products {
    pub fn new(db: Db) -> Products {
        Products {
            db
        }
    }

    pub async fn _create_product(&self, product: Product) {
        let query = sqlx::query("INSERT INTO products (name, description, categorie, price, count, image) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(product.name)
        .bind(product.description)
        .bind(product.categorie)
        .bind(product.price)
        .bind(product.count)
        .bind(product.image)
        .execute(&self.db.pool)
        .await;

        if query.is_err() {
            println!("error creating item");
            return;
        }

        println!("item created");
    }

    pub async fn _get_products(&self) -> Vec<Product> {
        let products: Result<Vec<Product>, sqlx::Error> = sqlx::query_as("SELECT * FROM products")
            .fetch_all(&self.db.pool)
            .await;

        if products.is_err() {
            println!("was error create {:?}", products.err());
            return vec![];
        }

        products.unwrap()
    }

}