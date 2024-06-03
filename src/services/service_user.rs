use crate::errors::error::Error;
use crate::database::db::AppState;
use crate::models::model_user::{ User, CreateUserPayload, LoginPayload, UpdateUserPayload };
use sqlx::{ FromRow, Postgres, Transaction, Pool };
use std::sync::Arc;
use uuid::Uuid;
use bcrypt::{ DEFAULT_COST, hash, verify };

pub async fn fetch_user_by_id(id: Uuid, app_state: &Pool<Postgres>) -> Result<User, Error> {
    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(app_state).await
        .map_err(|err| {
            let error_message = format!("Database query failed: {}", err);
            println!("{}", error_message);
            Error::GetUserError(error_message)
        })?;

    Ok(user)
}

pub async fn login(payload: &LoginPayload, app_state: &Pool<Postgres>) -> Result<User, Error> {
    let user = sqlx
        ::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(app_state).await
        .map_err(|err| {
            let error_message = format!("Database query failed: {}", err);
            println!("{}", error_message);
            Error::GetUserError("User not found.".to_string())
        })?;

    verify_password(&payload.password, &user.password).map(|_| user)
}

pub fn verify_password(input: &str, stored: &str) -> Result<(), Error> {
    verify(input, stored)
        .map_err(|_| Error::LoginError("Password verification failed.".to_string()))
        .and_then(|is_valid| {
            if is_valid { Ok(()) } else { Err(Error::LoginError("Invalid password.".to_string())) }
        })
}

pub fn validate(payload: &CreateUserPayload) -> Result<(), Error> {
    if payload.email.len() < 5 || !payload.email.contains('@') || !payload.email.contains('.') {
        return Err(Error::CreateUserError("Email is invalid.".to_string()));
    }

    if payload.name.len() < 2 {
        return Err(Error::CreateUserError("Name is too short.".to_string()));
    }

    if payload.password.len() < 8 {
        return Err(Error::CreateUserError("Password is too short.".to_string()));
    }

    Ok(())
}
