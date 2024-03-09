use serde::Serialize;

use crate::enums::Status;


#[derive(Serialize)]
pub struct NormResponse<T> {
    status: Status,
    data: T,
}

pub fn parse_response<T>(data: T, status: Status) -> NormResponse<T> {
    NormResponse { status, data }
}
