use axum::{
    Extension, Router,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};

use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::helpers::errors::AppError;
use crate::users::{
    model::{User, UserInput},
    service,
};

pub fn routes() -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_all_users))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
}

async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UserInput>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    let user = service::create_user(&pool, payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = service::get_user(&pool, id).await?;

    Ok(Json(user))
}

async fn get_all_users(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, AppError> {
    let users: Vec<User> = service::get_all_users(&pool).await?;
    Ok(Json(users))
}

async fn update_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserInput>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    let user = service::update_user(&pool, id, payload).await?;
    Ok(Json(user))
}

async fn delete_user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_user(&pool, id).await?;
    Ok("User deleted successfully")
}
