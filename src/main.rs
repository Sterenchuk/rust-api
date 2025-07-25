mod db;
mod helpers;
mod users;

use std::env;
use std::time::Duration;

use axum::Router;

// --- AppConfig struct and implementation (as you provided) ---
#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: Vec<u8>,
    pub jwt_expiration: Duration,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self, String> {
        // Corrected: env var name should be "DATABASE_URL" not "_URL"
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;

        let jwt_secret_str =
            env::var("JWT_SECRET").map_err(|_| "JWT_SECRET must be set".to_string())?;
        let jwt_secret = jwt_secret_str.as_bytes().to_vec();

        let jwt_expiration_str = env::var("JWT_EXPIRETION_TIME")
            .map_err(|_| "JWT_EXPIRETION_TIME must be set".to_string())?;
        let jwt_expiration: f64 = jwt_expiration_str
            .parse()
            .map_err(|_| "JWT_EXPIRETION_TIME must be a valid number (e.g., 0.5, 2.0)".to_string())?;
        let jwt_expiration = Duration::from_secs_f64(jwt_expiration * 3600.0); // Convert hours to seconds

        Ok(AppConfig {
            database_url,
            jwt_secret,
            jwt_expiration,
        })
    }
}
// --- End of AppConfig ---


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); 

    let config = match AppConfig::load_from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            eprintln!("Please ensure DATABASE_URL, JWT_SECRET, and JWT_EXPIRETION_TIME are set.");
            std::process::exit(1); // Exit if configuration can't be loaded
        }
    };

    let pool = db::connection::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    let app = Router::new()
        .merge(users::controller::routes())
        .layer(axum::Extension(pool))
        .layer(axum::Extension(config));

    println!("Server running on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}