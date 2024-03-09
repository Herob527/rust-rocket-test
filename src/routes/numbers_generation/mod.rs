use rand::Rng;
use rocket::serde::{json::to_string, Serialize};

use crate::enums::Status;

use super::utils::parse_response;

#[derive(Serialize)]
struct RandomResult {
    generated_number: u8,
    numbers: String,
}

#[derive(Serialize)]
struct RoutesSchema<'a> {
    routes: Vec<&'a str>,
}

#[get("/")]
fn numbers_index() -> String {
    let binding = routes();
    let available_routes = RoutesSchema {
        routes: binding
            .iter()
            .map(|route| route.uri.path())
            .filter(|uri| uri != &"/")
            .collect::<Vec<&str>>(),
    };
    let response = parse_response(available_routes, Status::SUCCESS);
    to_string(&response).unwrap()
}

#[get("/<number>?<sep>")]
fn numbers_specific(number: u8, sep: Option<String>) -> String {
    if number == 0 {
        return to_string(&parse_response(
            "Number should be greater than 0",
            Status::ERROR,
        ))
        .unwrap();
    }
    let range_of_numbers = (1..(number + 1))
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>()
        .join(&sep.unwrap_or("".to_string()));
    let response = parse_response(range_of_numbers, Status::SUCCESS);
    to_string(&response).unwrap()
}

#[get("/rand?<sep>")]
fn number_random(sep: Option<String>) -> String {
    let num = rand::thread_rng().gen_range(1..=9);
    let range_of_numbers = (1..(num + 1))
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>()
        .join(&sep.unwrap_or("".to_string()));

    let response_body = RandomResult {
        generated_number: num,
        numbers: range_of_numbers,
    };

    let response = parse_response(response_body, Status::SUCCESS);
    to_string(&response).unwrap()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![numbers_index, numbers_specific, number_random]
}
