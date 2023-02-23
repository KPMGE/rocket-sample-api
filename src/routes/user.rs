use rocket::serde::json::Json;
use crate::{models::user::User, services};

#[get("/users")]
pub fn get_users() -> Json<Vec<User>> {
    Json(services::user::get_users())
}
