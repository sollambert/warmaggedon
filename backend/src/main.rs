use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::Router;
use futures::lock::Mutex;

use crate::types::state::{Room, RoomState};

mod controllers;
mod types;

#[tokio::main]
async fn main() -> () {
    let rooms = Mutex::new(HashMap::<String, Room>::new());
    let room_state = Arc::new(RoomState{rooms});

    let app = Router::new()
        .nest("/room", controllers::room_controller::routes())
        .with_state(room_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await {
        Ok(_) => {},
        Err(error) => panic!("Could not bind to {}: {}", addr,error)
    }
}