use generate_readme::generate_readme;
use rocket::Route;
use routes::numbers_generation;

#[macro_use]
extern crate rocket;

pub mod enums;
pub mod routes;

mod generate_readme;


#[launch]
fn rocket() -> _ {
    let rock = rocket::build()
        .mount("/", routes::routes())
        .mount("/numbers_generation", numbers_generation::routes());
    
    let routes = rock.routes().collect::<Vec<&Route>>();

    generate_readme(routes);

    rock
}
