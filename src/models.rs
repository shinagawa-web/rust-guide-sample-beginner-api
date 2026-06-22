use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}
