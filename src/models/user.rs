use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInput{
    pub name: String,
}
