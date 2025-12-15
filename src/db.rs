use std::env;

use mongodb::{Client, Database};

pub async fn connect_db() -> mongodb::error::Result<Database> {
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let db_name = env::var("MONGODB_DB_NAME")
        .unwrap_or_else(|_| "todo_db".to_string());

    let client = Client::with_uri_str(&mongodb_uri).await?;

    Ok(client.database(&db_name))
}