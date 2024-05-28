use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChatHandshake {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChatMessage {
    pub username: String,
    pub message: String
}