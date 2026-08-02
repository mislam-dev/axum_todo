use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn connect_db(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

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
                tracing::warn!(
                    "Database connection failed. Retrying in 3 seconds... (Attempt {}/{})",
                    retries,
                    max_retries
                );
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
}
