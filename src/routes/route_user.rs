use crate::errors::error::Error;
use crate::database::db::AppState;
use crate::models::model_user::User;
use crate::controllers::controller_user::{ create_user, get_user_by_id };
use crate::services::service_auth::auth;

use std::sync::Arc;
use serde::{ Deserialize, Serialize };
use axum::{
    extract::{ Path, Query, State },
    response::{ IntoResponse, Response },
    routing::{ get, post, Router },
    Json,
    middleware,
};

pub fn user_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/user", post(create_user))
        .route(
            "/api/user/:id",
            get(get_user_by_id).route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
        .with_state(app_state)
}
