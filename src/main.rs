mod app;
mod database;
mod modules;

use app::app;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", "0.0.0.0:3000");
    let _ = axum::serve(listener, app).await.unwrap();
}
