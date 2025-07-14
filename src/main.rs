mod helpers;
mod users;

use axum::{Router};

#[tokio::main]
async fn main() {
    let app = Router::new().merge(users::controller::routes());
    println!("Server running on 0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
