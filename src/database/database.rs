use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn connect_db(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let max_connections: u32 = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);
    let min_connections: u32 = std::env::var("DB_MIN_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);
    let connect_timeout_secs: u64 = std::env::var("DB_CONNECT_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    let idle_timeout_secs: u64 = std::env::var("DB_IDLE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(max_connections)
        .min_connections(min_connections)
        .connect_timeout(Duration::from_secs(connect_timeout_secs))
        .idle_timeout(Duration::from_secs(idle_timeout_secs))
        .sqlx_logging(cfg!(debug_assertions));

    let max_retries = 5;
    let mut retries = 0;

    loop {
        match Database::connect(opt.clone()).await {
            Ok(db) => {
                tracing::info!("Successfully db connected!");
                return Ok(db);
            }
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    tracing::error!(
                        "Failed to connect to database after {} attempts",
                        max_retries
                    );
                    return Err(e);
                }
                let backoff_secs = (2u64).pow(retries).min(30);
                tracing::warn!(
                    retries,
                    max_retries,
                    backoff_secs,
                    error = %e,
                    "Database connection failed. Retrying",
                );
                tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
            }
        }
    }
}
