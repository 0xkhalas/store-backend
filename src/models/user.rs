use sqlx::prelude::FromRow;


#[derive(FromRow, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub token: String
}
