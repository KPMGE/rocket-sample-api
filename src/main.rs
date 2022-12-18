#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn greet(name: &str, age: u8) -> String {
    format!("Hello, {} your age is {}, isn't it?", name, age)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, greet])
}
