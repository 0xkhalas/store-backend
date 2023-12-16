use sqlx::query::QueryAs;

use crate::models::{self, user::User};

use super::db::Db;



pub struct Users {
    pub db: Db
}

impl Users {
    pub fn new(db: Db) -> Users {
        Users {
            db
        }
    }

    pub async fn get_users(&self) -> Vec<User> {
        let users: Result<Vec<User>, sqlx::Error> = sqlx::query_as("SELECT id, email, password, token  FROM users")
            .fetch_all(&self.db.pool)
            .await;
        
        println!("{:?}", users);

        vec![]
    }

    pub async fn create_user(&self, user: models::user::User) {
        let query = sqlx::query("INSERT INTO users(email, password, token) VALUES (?,?,?)")
            .bind(user.email)
            .bind(user.password)
            .bind(user.token)
            .execute(&self.db.pool)
            .await
            .expect("User not create");

    }
}