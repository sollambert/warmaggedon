use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChatHandshake<'ch> {
    pub username: &'ch str,
    pub room_id: i64,
    pub password: &'ch str
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChatMessage<'ch> {
    pub username: &'ch str,
    pub room_id: i64,
    pub message: &'ch str
}