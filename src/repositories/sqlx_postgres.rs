use sqlx::{PgPool, postgres::{PgPoolOptions, PgRow}, Row};
use crate::models::user::User;

#[derive(Debug, Clone)]
pub struct SqlxHelper {
    pool: PgPool
}

impl SqlxHelper {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(db_url).await {
                Ok(pool) => pool,
                Err(e) => panic!("could not connect to database: {:?}", e)
            };

        SqlxHelper { pool: db_pool }
    }

    pub async fn get_users(self) -> Result<Vec<User>, sqlx::Error> {
        match sqlx::query("SELECT * FROM users")
        .map(|row: PgRow| User {
            name: row.get("name"),
            age: row.get("age"),
            email: row.get("email"),
            password: row.get("password")
        }).fetch_all(&self.pool).await {
            Ok(questions) => Ok(questions),
            Err(e) => Err(e)
        }
    }

    pub async fn insert_user(self, user: User) -> Result<User,  sqlx::Error> {
        match sqlx::query("INSERT INTO users (name, age, email, password) VALUES($1, $2, $3, $4) RETURNING name, age, email, password")
            .bind(user.name)
            .bind(user.age)
            .bind(user.email)
            .bind(user.password)
            .map(|row: PgRow| User {
                name: row.get("name"),
                age: row.get("age"),
                email: row.get("email"),
                password: row.get("password")
            })
            .fetch_one(&self.pool).await {
                Ok(user) => Ok(user),
                Err(e) => Err(e)
        }
    }
}
