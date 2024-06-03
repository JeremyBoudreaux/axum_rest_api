extern crate bcrypt;

use crate::errors::error::Error;
use serde::{ Deserialize, Serialize };
use sqlx::{ FromRow, Postgres, Transaction, Pool };
use crate::services::service_user::validate;
use uuid::Uuid;
use bcrypt::{ DEFAULT_COST, hash, verify };
use crate::database::db::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPayload {
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn new(email: String, name: String, password: String) -> Result<Self, Error> {
        validate(
            &(CreateUserPayload {
                email: email.clone(),
                name: name.clone(),
                password: password.clone(),
            })
        )?;

        let hashed_pw = hash(password, DEFAULT_COST);

        match hashed_pw {
            Ok(pw) => Ok(Self { id: Uuid::new_v4(), email, name, password: pw }),
            Err(_) => Err(Error::CreateUserError("Password hashing failed.".to_string())),
        }
    }

    pub async fn save(&self, session: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
        sqlx
            ::query("INSERT INTO users (id, email, name, password) VALUES ($1, $2, $3, $4)")
            .bind(&self.id)
            .bind(&self.email)
            .bind(&self.name)
            .bind(&self.password)
            .execute(session).await
            .map_err(|err| {
                let error_message = format!("Database insert failed: {}", err);
                println!("{}", error_message);
                Error::CreateUserError(error_message)
            })?;

        Ok(())
    }

    pub async fn update(&self, session: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
        sqlx
            ::query("UPDATE users SET email = $1, name = $2, password = $3 WHERE id = $4")
            .bind(&self.email)
            .bind(&self.name)
            .bind(&self.password)
            .bind(&self.id)
            .execute(session).await
            .map_err(|err| {
                let error_message = format!("Database update failed: {}", err);
                println!("{}", error_message);
                Error::UpdateUserError(error_message)
            })?;

        Ok(())
    }
}
