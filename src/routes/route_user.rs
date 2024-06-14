use crate::database::db::AppState;
use crate::controllers::controller_user::{ create_user, get_user_by_id, update_user };
use crate::services::service_auth::auth;

use std::sync::Arc;
use axum::{ routing::{ get, post, put, Router }, middleware };

pub fn user_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/user", post(create_user))
        .route(
            "/api/user/:id",
            get(get_user_by_id).route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
        .route(
            "/api/user/:id",
            put(update_user).route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        )
        .with_state(app_state)
}
