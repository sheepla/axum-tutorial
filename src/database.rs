use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    Database::connect(&database_url).await
}
