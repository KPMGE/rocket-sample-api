use rocket::serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct User {
    pub name: String, 
    pub age: i32,
    pub email: String, 
    pub password: String
}
