use crate::helpers::{
    errors::AppError,
    hash};
use crate::users::{
    model::{User, UserInput},
    repository,
};
use sqlx::PgPool;
use uuid::Uuid;


pub async fn create_user(pool: &PgPool, user: UserInput) -> Result<User, AppError> {
    match repository::get_user_by_email(pool, &user.email).await {
        Ok(_) => {
            return Err(AppError::Conflict);
        }
        Err(AppError::NotFound) => {}
        Err(e) => {
            eprintln!("Unexpected error checking user existence: {:?}", e);
            return Err(AppError::Internal);
        }
    }

    let hash = hash::hash_password(&user.password).map_err(|_| AppError::Internal)?; 

    match repository::create_user(pool, &user.name, &user.email, &hash).await {
        Ok(user) => Ok(user),
        Err(AppError::NotFound) => Err(AppError::NotFound),
        Err(e) => {
            eprintln!("Unexpected error creating user: {:?}", e);
            Err(AppError::Internal)
        }
    }
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    match repository::get_all_users(pool).await {
        Ok(users) => Ok(users),
        Err(AppError::NotFound) => Err(AppError::NotFound),
        Err(e) => {
            eprintln!("Unexpected error creating user: {:?}", e);
            Err(AppError::Internal)
        }
    }
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
    match repository::get_user(pool, id).await {
        Ok(user) => Ok(user),
        Err(AppError::NotFound) => Err(AppError::NotFound),
        Err(e) => {
            eprintln!("Unexpected error creating user: {:?}", e);
            Err(AppError::Internal)
        }
    }
}

pub async fn update_user(pool: &PgPool, id: Uuid, user: UserInput) -> Result<User, AppError> {

    let hash = hash::hash_password(&user.password).map_err(|_| AppError::Internal)?;

    match repository::update_user(pool, id, &user.name, &user.email, &hash).await {
        Ok(user) => Ok(user),
        Err(AppError::NotFound) => Err(AppError::NotFound),
        Err(e) => {
            eprintln!("Unexpected error creating user: {:?}", e);
            Err(AppError::Internal)
        }
    }
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    match repository::delete_user(pool, id).await {
        Ok(()) => Ok(()),
        Err(AppError::NotFound) => Err(AppError::NotFound),
        Err(e) => {
            eprintln!("Unexpected error creating user: {:?}", e);
            Err(AppError::Internal)
        }
    }
}
