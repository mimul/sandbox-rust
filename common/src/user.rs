use serde::Serialize;
#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub status: bool,
}