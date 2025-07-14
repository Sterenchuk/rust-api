use axum::{
    Router,
    extract::{Json, Path},
    response::IntoResponse,
    routing::{delete, get, post, put},
    http::StatusCode,
};
use serde::Deserialize;

use crate::helpers::errors::AppError;
use crate::users::{model::User, service};

#[derive(Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_all_users))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
}

async fn create_user(
    Json(payload): Json<UserInput>
) -> Result<impl IntoResponse, AppError> {
    let user = service::create_user(payload.name, payload.email, payload.password)?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn get_user(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let user = service::get_user(id)?;
    Ok(Json(user))
}

async fn get_all_users() -> Result<impl IntoResponse, AppError> {
    let users:Vec<User> = service::get_all_users()?;
    Ok(Json(users))
}

async fn update_user(
    Path(id): Path<String>,
    Json(payload): Json<UserInput>,
) -> Result<impl IntoResponse, AppError> {
    let user = service::update_user(id, payload.name, payload.email, payload.password)?;
    Ok(Json(user))
}

async fn delete_user(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    service::delete_user(id)?;
    Ok("User deleted successfully")
}
