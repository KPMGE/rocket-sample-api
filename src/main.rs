use rocket::serde::{Serialize, json::Json};

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn greet(name: &str, age: u8) -> String {
    format!("Hello, {} your age is {}, isn't it?", name, age)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Person {
    name: String,
    age: i32
}

#[get("/person")]
fn get_person() -> Json<Person> {
    Json(Person {  
        name: "kevin".to_string(),
        age: 21
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, greet, get_person])
}
