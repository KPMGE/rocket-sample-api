mod routes;
mod models;
mod services;
mod repositories;

use crate::routes::user;

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}


#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, user::get_users, user::insert_user])
}
