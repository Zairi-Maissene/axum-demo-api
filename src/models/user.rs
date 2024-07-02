use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
