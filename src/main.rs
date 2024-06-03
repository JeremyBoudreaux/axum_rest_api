#![allow(unused)]
mod routes;
mod database;
mod errors;
mod models;
mod controllers;
mod services;

use crate::services::service_auth::auth;
use database::db;
use std::sync::Arc;
use axum::{ Router, serve };

#[tokio::main]
async fn main() {
    let pool = db::connect().await;
    let migrations = db::migrate(&pool).await;

    let app_state = Arc::new(db::AppState { db: pool.clone() });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    let app_routes = Router::new()
        .merge(routes::route_user::user_route(app_state.clone()))
        .merge(routes::route_auth::auth_route(app_state.clone()));

    serve(listener, app_routes).await.unwrap();
}
