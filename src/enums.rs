use serde::Serialize;

#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    ERROR,
}
