use crate::errors::error::Error;
use crate::database::db::AppState;
use crate::models::model_user::User;
use crate::controllers::controller_auth::{ login_user, logout_user };
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

pub fn auth_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_user))
        .route("/api/auth/logout", get(logout_user))
        .with_state(app_state)
}
