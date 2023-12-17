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

    pub async fn _get_users(&self) -> Vec<User> {
        let users: Result<Vec<User>, sqlx::Error> = sqlx::query_as("SELECT id, email, password, token  FROM users")
            .fetch_all(&self.db.pool)
            .await;
        
        if users.is_err() {
            println!("was error creating {:?}", users.err());
            return vec![];
        }
        
        users.unwrap()
    }

    pub async fn create_user(&self, user: models::user::User) -> bool {
        let query = sqlx::query("INSERT INTO users(email, password, token) VALUES (?,?,?)")
            .bind(user.email)
            .bind(user.password)
            .bind(user.token)
            .execute(&self.db.pool)
            .await;

        match query {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub async fn get_user(&self, email: &String) -> Option<User> {
        let query: Result<User, sqlx::Error> = sqlx::query_as("SELECT * FROM users WHERE email=?")
            .bind(email)
            .fetch_one(&self.db.pool)
            .await;

        match query {
            Ok(e) => Some(e),
            Err(_) => None
        }
    } 

    pub async fn is_exists(&self, email: &String) -> bool {
        match self.get_user(email).await {
            Some(_) => true,
            None => false
        }
    }
}