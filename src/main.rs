mod routes;
mod models;
mod services;
mod repositories;

use repositories::sqlx_postgres::SqlxHelper;
use crate::routes::user;

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}


#[launch]
async fn rocket() -> _ {
    let db_url = "postgres://postgres:1234@localhost:5432/users";
    let helper = SqlxHelper::new(db_url).await.expect("could not connect to the database");

    println!("database connected successfully...");
    rocket::build()
        .manage::<SqlxHelper>(helper)
        .mount("/", routes![hello, user::get_users, user::insert_user])
}
