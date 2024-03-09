pub mod numbers_generation;
pub mod utils;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}


pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
