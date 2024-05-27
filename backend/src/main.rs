use std::net::SocketAddr;

use axum::Router;

mod controllers;
mod types;

#[tokio::main]
async fn main() -> () {
    let app = Router::new()
        .nest("/ws", controllers::ws_controller::routes());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await {
        Ok(_) => {},
        Err(error) => panic!("Could not bind to {}: {}", addr,error)
    }

}