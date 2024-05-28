use std::collections::HashMap;

use futures::lock::Mutex;
use tokio::sync::broadcast;

pub struct RoomState {
    pub rooms: Mutex<HashMap<String, Room>>
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub creator: String,
    pub password: String,
    pub users: Vec<String>,
    pub tx: broadcast::Sender<String>
}
