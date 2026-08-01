mod app;
mod database;
mod error;
mod modules;

use app::app;
use dotenvy::dotenv;
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", "0.0.0.0:3000");
    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Ctrl+C received! Shutting down gracefully...");
        },
        _ = terminate => {
            println!("Terminate signal received! Shutting down gracefully...");
        },
    }
}
