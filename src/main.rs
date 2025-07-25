mod db;
mod helpers;
mod users;

use axum::Router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = db::connection::create_pool(&database_url)
        .await
        .expect("DB pool");

    let app = Router::new()
        .merge(users::controller::routes())
        .layer(axum::Extension(pool));

    println!("Server running on port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
