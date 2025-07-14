use crate::{helpers::errors::AppError, users::model::User, users::repository};
use uuid::Uuid;

pub fn create_user(name: String, email: String, password: String) -> Result<User, AppError> {
    let user = User {
        id: Uuid::new_v4(),
        name,
        email,
        password,
    };

    repository::create_user(user)
}

pub fn get_all_users() -> Result<Vec<User>, AppError> {
    repository::get_all_users()
}

pub fn get_user(id: String) -> Result<User, AppError> {
    repository::get_user(&id)
}

pub fn update_user(
    id: String,
    name: String,
    email: String,
    password: String,
) -> Result<User, AppError> {
    let user = User {
        id: Uuid::parse_str(&id).map_err(|_| AppError::NotFound)?,
        name,
        email,
        password,
    };

    repository::update_user(&id, user)
}

pub fn delete_user(id: String) -> Result<(), AppError> {
    repository::delete_user(&id)
}
