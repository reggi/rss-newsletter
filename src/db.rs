use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;

#[derive(Debug)]
#[derive(sqlx::FromRow)]
pub struct Subscriber {
    pub email: String,
    pub unsubscribe: bool,
}

#[derive(Debug)]
#[derive(sqlx::FromRow)]
pub struct FeedItem {
    pub feed_url: String,
    pub item_guid: String,
    pub all_emails_sent: bool,
    pub skip: bool,
}

pub async fn get_connection(path: String) -> Result<SqlitePool, sqlx::Error> {
    println!("ensuring database file exists {}", path);
    
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!().run(&pool).await?;
    println!("Tables created successfully.");
    Ok(pool)
}