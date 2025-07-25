use crate::helpers::errors::AppError;
use crate::users::model::User;
use sqlx::Error as SqlxError;
use sqlx::PgPool;
use uuid::Uuid;

/// Create a user in the database.
pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, password
        "#,
        name,
        email,
        password,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| AppError::DbError(err.into()))?;

    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    let users = sqlx::query_as!(User, "SELECT id, name, email, password FROM users")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DbError(e.into()))?;

    Ok(users)
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DbError(e.into()))?;

    Ok(user)
}

pub async fn update_user(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    email: &str,
    password: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users SET name = $1, email = $2, password = $3
        WHERE id = $4
        RETURNING id, name, email, password
        "#,
        name,
        email,
        password,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DbError(e.into()))?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DbError(e.into()))?;

    Ok(())
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
    let result = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password FROM users WHERE email = $1"#,
        email
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(user) => Ok(user),
        Err(SqlxError::RowNotFound) => Err(AppError::NotFound),
        Err(e) => Err(AppError::DbError(e)),
    }
}
