mod app;
mod modules;

use app::app;

#[tokio::main]
async fn main() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", "0.0.0.0:3000");
    let _ = axum::serve(listener, app).await.unwrap();
}
