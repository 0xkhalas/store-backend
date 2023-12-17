#[derive(Clone)]
pub struct Db {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

impl Db {
    pub async fn new() -> Db {
        // Create Database
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite").create_if_missing(true))
            .await
            .expect("[!] Database Not Created");

        Db {
            pool: pool
        }
    }

    pub async fn init_tables(&self) {
        // Create Tables

        // users
        sqlx::query(
            r"create table if not exists users (
                email varchar(64),
                password varchar(64),
                token varchar(64)
           )",
        )
        .execute(&self.pool)
        .await
        .expect("Database can't create table messages");

        // products
        sqlx::query(
            r"create table if not exists products (
               name varchar(64),
               description varchar(64),
               categorie varchar(64),
               price int,
               count int,
               image text
           )",
        )
        .execute(&self.pool)
        .await
        .expect("Database can't create table messages");

    }
}
