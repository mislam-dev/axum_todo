use sea_orm::{Database, DatabaseConnection};

pub async fn connect_db(db_url: &str) -> DatabaseConnection {
    let db = Database::connect(db_url).await.unwrap();

    db
}
