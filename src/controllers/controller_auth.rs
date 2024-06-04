use crate::errors::error::Error;
use crate::database::db::AppState;
use crate::services::service_auth::{
    create_access_token,
    create_refresh_token,
    parse_cookies_from_request,
};
use crate::models::model_user::{ User, LoginPayload };
use crate::services::service_user::{ login, logout };
use std::sync::Arc;
use uuid::Uuid;

use axum::{
    extract::{ Path, Query, State },
    http::{ StatusCode, HeaderMap, header, Request },
    response::{ IntoResponse, Response },
    Json,
    body::Body,
};

// @route POST /auth/login
// @desc Login user
// @access Public
pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>
) -> Result<impl IntoResponse, Error> {
    let user = login(&payload, &app_state.db).await?;

    let access_token = create_access_token(&user.id);
    let refresh_token = create_refresh_token(&user.id).await;

    match access_token {
        Ok(access_token) => {
            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                format!("access_token={}; Secure; HttpOnly; Path=/; SameSite=Strict", access_token)
                    .parse()
                    .unwrap()
            );

            headers.append(
                header::SET_COOKIE,
                format!(
                    "refresh_token={}; Secure; HttpOnly; Path=/; SameSite=Strict",
                    refresh_token.unwrap()
                )
                    .parse()
                    .unwrap()
            );

            Ok((StatusCode::OK, headers, Json(user)))
        }
        Err(e) => Err(Error::LoginError(e.to_string())),
    }
}

// @route GET /auth/logout
// @desc Logout user
// @access Private
pub async fn logout_user(
    State(app_state): State<Arc<AppState>>,
    request: Request<Body>
) -> Result<impl IntoResponse, Error> {
    let mut headers = HeaderMap::new();

    let cookies = parse_cookies_from_request(&request)?;

    logout(cookies.refresh_token, &app_state.db).await?;

    headers.append(
        header::SET_COOKIE,
        "access_token=; Secure; HttpOnly; Path=/; SameSite=Strict; Max-Age=0".parse().unwrap()
    );

    headers.append(
        header::SET_COOKIE,
        "refresh_token=; Secure; HttpOnly; Path=/; SameSite=Strict; Max-Age=0".parse().unwrap()
    );

    Ok((StatusCode::NO_CONTENT, headers))
}
