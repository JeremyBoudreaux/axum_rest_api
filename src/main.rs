#![allow(unused)]
mod routes;
mod database;
mod errors;
mod models;
mod controllers;
mod services;

use crate::services::service_auth::auth;
use axum::routing::get;
use socketioxide::{ extract::SocketRef, layer, SocketIo };
use database::db;
use std::sync::Arc;
use axum::{ Router, serve };
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    println!("Socket connected: {:?}", socket.id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let pool = db::connect().await;
    let migrations = db::migrate(&pool).await;

    let app_state = Arc::new(db::AppState { db: pool.clone() });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    let app_routes = Router::new()
        .merge(routes::route_user::user_route(app_state.clone()))
        .merge(routes::route_auth::auth_route(app_state.clone()))
        .layer(layer);

    serve(listener, app_routes).await.unwrap();

    Ok(())
}
