use crate::errors::error::Error;
use crate::database::db::AppState;
use crate::models::model_user::{ User, CreateUserPayload };
use crate::services::service_user::fetch_user_by_id;
use std::sync::Arc;
use uuid::Uuid;

use axum::{ extract::{ Path, State }, http::StatusCode, Json };

// @route POST /users
// @desc Create a new user
// @access Public
pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>
) -> Result<(StatusCode, Json<User>), Error> {
    let user = User::new(payload.email, payload.name, payload.password)?;

    let mut session = app_state
        .clone()
        .db.begin().await
        .map_err(|_| Error::CreateUserError("Database connection failed.".to_string()))?;

    match user.save(&mut session).await {
        Ok(_) => {
            session
                .commit().await
                .map_err(|_| Error::CreateUserError("Database commit failed.".to_string()))?;
        }
        Err(_) => {
            session.rollback().await.unwrap();
            return Err(Error::CreateUserError("Database insert failed.".to_string()));
        }
    }

    Ok((StatusCode::CREATED, Json(user)))
}

// @route GET /users/:id
// @desc Get user by ID
// @access Public
pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<User>), Error> {
    let user = fetch_user_by_id(id, &app_state.db).await?;

    Ok((StatusCode::OK, Json(user)))
}

// @route PUT /users/:id
// @desc Update user by ID
// @access Public
pub async fn update_user(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<User>), Error> {
    let user = fetch_user_by_id(id, &app_state.db).await?;

    Ok((StatusCode::OK, Json(user)))
}
