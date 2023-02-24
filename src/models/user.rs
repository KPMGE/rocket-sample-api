use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String, 
    pub age: i32,
    pub email: String, 
    pub password: String
}
