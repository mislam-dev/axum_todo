mod app;
mod core;
mod database;
mod modules;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use app::app;
use dotenvy::dotenv;
use tokio::signal;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // Default to "info" level for third-party crates, but "debug" for our own app
                .unwrap_or_else(|_| "info,axum_todo=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting up the application...");
    let app = app().await;

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let addr = SocketAddr::new(ip, 3000);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let local_addr = listener.local_addr().unwrap();
    tracing::info!("Listening on {}", local_addr);

    let _ = axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .ok();
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
