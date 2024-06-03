use serde::{ Deserialize, Serialize };
use axum::{
    extract::{ Path, Query, State },
    http::StatusCode,
    response::{ IntoResponse, Response },
    routing::{ get, post, Router },
    Json,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    CreateUserError(String),
    GetUserError(String),
    LoginError(String),
    UpdateUserError(String),
    Unauthorized(String),
    InvalidToken,
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::CreateUserError(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            Error::GetUserError(message) => { (StatusCode::NOT_FOUND, message).into_response() }
            Error::LoginError(message) => { (StatusCode::UNAUTHORIZED, message).into_response() }
            Error::UpdateUserError(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            Error::Unauthorized(message) => { (StatusCode::UNAUTHORIZED, message).into_response() }
            Error::InvalidToken => { (StatusCode::UNAUTHORIZED, "Invalid token").into_response() }
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}
