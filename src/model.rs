use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signup {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub token: String,
}

#[derive(FromRow, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Claims {
    // email
    pub sub: String,
    pub exp: usize,
}
