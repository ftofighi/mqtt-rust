use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    // Connect to SQLite database (file will be created if it doesn’t exist)
    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://./mqtt_messages.db")
        .await?;

    println!("✅ Connected to database");

    Ok(())
}
