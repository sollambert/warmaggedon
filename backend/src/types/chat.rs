use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChatHandshake<'ch> {
    username: &'ch str,
    room_id: i64,
    password: &'ch str
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChatMessage<'ch> {
    username: &'ch str,
    room_id: i64,
    message: &'ch str
}