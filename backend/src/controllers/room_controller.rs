use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
// use std::sync::{Arc, Mutex};
use axum::extract::{ConnectInfo, Path};
use axum::Json;
use axum::{
    routing::{get, post},
    Router
};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}, response::IntoResponse
};
use futures::lock::Mutex;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Deserialize;
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};

use crate::types::chat::ChatHandshake;

struct RoomState {
    rooms: Mutex<HashMap<String, Room>>
}

#[derive(Debug, Clone)]
struct Room {
    pub id: String,
    pub creator: String,
    pub password: String,
    pub users: Vec<String>,
    tx: broadcast::Sender<String>
}

// route function to nest endpoints in router
pub fn routes() -> Router {
    let rooms = Mutex::new(HashMap::<String, Room>::new());
    let app_state = Arc::new(RoomState{rooms});
    // create routes
    Router::new()
        .route("/create", post(create_room))
        .route("/join/:room_id", get(join_room_handler))
        .with_state(app_state)
}

fn gen_room_id() -> String {
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(8)
    .map(char::from)
    .collect()
}

async fn create_room(
        State(state): State<Arc<RoomState>>,
        Json(handshake): Json<ChatHandshake>
    ) -> impl IntoResponse {
    let mut rooms = state.rooms.lock().await;
    for room in rooms.clone().into_values() {
        if room.users.contains(&handshake.username) || room.creator == handshake.username {
            return format!("Already in a room {:?}", room.id);
        }
    }
    let mut id: String = gen_room_id();
    while rooms.contains_key(&id) {
        id = gen_room_id();
    }
    let (tx, _rx) = broadcast::channel(100);
    let mut new_room = Room {
        id: id.clone(),
        creator: handshake.username.clone(),
        password: handshake.password,
        users: Vec::new(),
        tx
    };
    new_room.users.push(handshake.username);
    rooms.insert(id.clone(), new_room.clone());
    for (key, value) in rooms.iter() {
        println!("{}: {:?}", key, value);
    }
    
    drop(rooms);
    return format!("Room created: {:?}", id);
}

#[derive(Deserialize)]
struct JoinParams {
    room_id: String
}

async fn join_room_handler(
        ws: WebSocketUpgrade,
        State(state): State<Arc<RoomState>>,
        connect_info: ConnectInfo<SocketAddr>,
        Path(JoinParams {room_id}): Path<JoinParams>
    ) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {join_room_handle_socket(socket, state, connect_info, room_id)})
}

async fn join_room_handle_socket(
        socket: WebSocket,
        state: Arc<RoomState>,
        connect_info: ConnectInfo<SocketAddr>,
        room_id: String
    ) {

    println!("processing new connection");

    let mut rooms = state.rooms.lock().await;
    for (key, value) in rooms.iter() {
        println!("{}: {:?}", key, value);
    }
    let room = rooms.get(&room_id).unwrap();
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();

    let mut rx = room.tx.subscribe();
    let tx = room.tx.clone();

    if let Some(Ok(init_msg)) = receiver.next().await {
        if let Message::Text(text) = init_msg {
            let handshake: ChatHandshake = serde_json::from_str(text.as_str()).unwrap_or_default();
            if handshake == ChatHandshake::default()
            || handshake.password != room.password {
                println!("{:?}", handshake);
                println!("Failed handshake from: {:?}", connect_info.clone().0);
                sender.close().await.unwrap();
                drop(rooms);
                return;
            }
            println!("{:?}", handshake);
            username = handshake.username.to_string();
        }
    }

    let join_msg = format!("{username} joined {}.", room.id);
    println!("{}", join_msg);
    let _ = room.tx.send(join_msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg == String::new() {
                continue;
            }
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if text == String::new() {
                continue;
            }
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let exit_msg = format!("{username} left.");
    println!("{}", exit_msg);
    let _ = room.tx.send(exit_msg);

    for room in rooms.clone().into_values() {
        if room.creator == username {
            rooms.remove(&room.id);
            break;
        } else if room.users.contains(&username) {
            let mut new_room = rooms.get(&room.id).unwrap().clone();
            let user_index = new_room.users.iter().position(|u| u.as_str() == username).unwrap();
            new_room.users.remove(user_index);
            rooms.insert(new_room.id.clone(), new_room.to_owned());
        }
    }
    drop(rooms);
}
