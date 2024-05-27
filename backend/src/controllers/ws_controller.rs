use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::extract::ConnectInfo;
use axum::{
    routing::get,
    Router
};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}, response::IntoResponse
};
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};

use crate::types::chat::ChatHandshake;

struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>
}

// route function to nest endpoints in router
pub fn routes() -> Router {
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState{user_set, tx});
    // create routes
    Router::new()
        .route("/", get(ws_handler))
        .with_state(app_state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, connect_info: ConnectInfo<SocketAddr>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {handle_socket(socket, state, connect_info)})
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, connect_info: ConnectInfo<SocketAddr>) {
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();
    if let Some(Ok(init_msg)) = receiver.next().await {
        if let Message::Text(text) = init_msg {
            let handshake: ChatHandshake = serde_json::from_str(text.as_str()).unwrap_or_default();
            if handshake == ChatHandshake::default() {
                println!("{:?}", handshake);
                println!("Failed handshake from: {:?}", connect_info.clone().0);
                return;
            }
            println!("{:?}", handshake);
            username = handshake.username.to_string();
        }
    }

    let mut rx = state.tx.subscribe();

    let join_msg = format!("{username} joined.");
    println!("{}", join_msg);
    let _ = state.tx.send(join_msg);

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

    let tx = state.tx.clone();
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
    let _ = state.tx.send(exit_msg);

    state.user_set.lock().unwrap().remove(&username);
}
