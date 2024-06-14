mod routes;
mod database;
mod errors;
mod models;
mod controllers;
mod services;

use axum::{ Router, serve };
use database::db;
use socketioxide::{ extract::SocketRef, SocketIo };
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    println!("Socket connected: {:?}", socket.id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let pool = db::connect().await;
    db::migrate(&pool).await;
    let app_state = Arc::new(db::AppState { db: pool.clone() });

    let cors = CorsLayer::permissive();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    let (io_layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app_routes = Router::new()
        .merge(routes::route_user::user_route(app_state.clone()))
        .merge(routes::route_auth::auth_route(app_state.clone()))
        .layer(ServiceBuilder::new().layer(cors).layer(io_layer));

    println!("Listening on http://{}", listener.local_addr().unwrap());
    serve(listener, app_routes).await.unwrap();

    Ok(())
}
