use crate::database::db::AppState;
use crate::controllers::controller_auth::{ login_user, logout_user };

use std::sync::Arc;
use axum::routing::{ get, post, Router };

pub fn auth_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_user))
        .route("/api/auth/logout", get(logout_user))
        .with_state(app_state)
}
