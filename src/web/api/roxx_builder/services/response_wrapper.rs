use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseWrapper<T: Serialize> {
    pub status: String,
    pub data: T,
}
