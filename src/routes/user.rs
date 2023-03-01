use rocket::{serde::json::Json, form::Form};
use crate::{
    models::user::User, 
    repositories::sqlx_postgres::SqlxHelper
};

#[get("/users")]
pub async fn get_users(helper: &rocket::State<SqlxHelper>) -> Json<Vec<User>> {
    let users = match helper.inner().get_users().await {
        Ok(users) => users,
        Err(e) => panic!("{e}")
    };

    Json(users)
}

#[post("/users", data = "<new_user>")]
pub async fn insert_user(helper: &rocket::State<SqlxHelper>, new_user: Json<User>) -> Json<User> {
    let user = match helper.inner().insert_user(new_user.into_inner()).await {
        Ok(user) => user,
        Err(e) => panic!("{e}")
    };

    Json(user)
}
