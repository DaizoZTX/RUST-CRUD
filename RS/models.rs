use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub ci: i32,
    pub nombre: String,
    pub galletas: i32,
}
