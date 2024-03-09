use routes::numbers_generation;


#[macro_use]
extern crate rocket;

pub mod enums;
pub mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::routes())
        .mount("/numbers_generation", numbers_generation::routes())
}
