use rocket::serde::json::Json;
use crate::{
    models::user::User, 
    repositories::sqlx_postgres::SqlxHelper
};

#[get("/users")]
pub async fn get_users() -> Json<Vec<User>> {
    // should move this connection to a single place
    let sqlx_helper = SqlxHelper::new("postgres://postgres:1234@localhost:5432/users").await;
    let users = match sqlx_helper.get_users().await {
        Ok(users) => users,
        Err(e) => panic!("{e}")
    };

    Json(users)
}

#[post("/user")]
pub async fn insert_user() -> Json<User> {
    // should move this connection to a single place
    let sqlx_helper = SqlxHelper::new("postgres://postgres:1234@localhost:5432/users").await;

    let user = match sqlx_helper.insert_user(User {
        name: "test user".to_string(),
        password: "test password".to_string(), 
        email: "test email".to_string(),
        age: 21
    }).await {
        Ok(user) => user,
        Err(e) => panic!("{e}")
    };

    Json(user)
}
