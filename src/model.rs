
use sqlx::SqlitePool;
use crate::db::{get_connection, FeedItem, Subscriber};
use crate::utils::clean_email;

#[derive(Clone)]
pub struct Model {
  pool: SqlitePool,
  feed_url: String
}

impl Model {
  // Constructor to create a new instance of `Model`
  pub async fn new(sqlite_file: &str, feed_url: &str) -> Result<Self, sqlx::Error> {
      println!("Creating model with file: {}", sqlite_file);
      let pool = get_connection(sqlite_file.to_string()).await?;
      println!("Created the connection");
      Ok(Self { pool, feed_url: feed_url.to_string() })
  }

  pub async fn get_subscribers(&self) -> Result<Vec<Subscriber>, sqlx::Error> {
    let subscribers = sqlx::query_as!(Subscriber,
        "SELECT email, unsubscribe FROM subscribers WHERE unsubscribe = false AND feed_url = ?1",
        self.feed_url
    )
    .fetch_all(&self.pool)
    .await?;
    Ok(subscribers)
  }

  pub async fn add_subscriber(&self, raw_email: &str) -> Result<usize, sqlx::Error> {
    let email = clean_email(&raw_email).expect("Invalid email address");
    let sql = r#"
        INSERT INTO subscribers (email, feed_url, unsubscribe) VALUES (?1, ?2, false)
        ON CONFLICT (email, feed_url) DO UPDATE SET unsubscribe = EXCLUDED.unsubscribe
    "#;
    let result = sqlx::query(sql)
        .bind(email)
        .bind(&self.feed_url)
        .execute(&self.pool)
        .await?
        .rows_affected();
    Ok(result as usize)
  }

  pub async fn unsubscribe(&self, raw_email: &str) -> Result<usize, sqlx::Error> {
    let email = clean_email(&raw_email).expect("Invalid email address");
    let sql = r#"
        INSERT INTO subscribers (email, feed_url, unsubscribe) VALUES (?1, ?2, 1)
        ON CONFLICT (email, feed_url) DO UPDATE SET unsubscribe = 1
    "#;
    let result = sqlx::query(sql)
        .bind(email)
        .bind(&self.feed_url)
        .execute(&self.pool)
        .await?
        .rows_affected();
    Ok(result as usize)
  }

  pub async fn mark_email_as_sent(&self, raw_email: &str, item_guid: &str) -> Result<usize, sqlx::Error> {
    let email = clean_email(&raw_email).expect("Invalid email address");
    let sql = r#"
        INSERT INTO emails_sent (email, item_guid, feed_url, success, failures) 
        VALUES (?1, ?2, ?3, true, 0)
        ON CONFLICT(feed_url, item_guid, email) DO UPDATE SET
        success = true
    "#;
    let result = sqlx::query(sql)
        .bind(email)
        .bind(item_guid)
        .bind(&self.feed_url)
        .execute(&self.pool)
        .await?
        .rows_affected();
    Ok(result as usize)
  }

  pub async fn mark_email_as_failure(&self, raw_email: &str, item_guid: &str) -> Result<usize, sqlx::Error> {
    let email = clean_email(&raw_email).expect("Invalid email address");
    let sql = r#"
        INSERT INTO emails_sent (email, item_guid, feed_url, success, failures) 
        VALUES (?1, ?2, ?3, false, 1)
        ON CONFLICT(feed_url, item_guid, email) DO UPDATE SET
        failures = failures + 1, success = false
    "#;
    let result = sqlx::query(sql)
        .bind(email)
        .bind(item_guid)
        .bind(&self.feed_url)
        .execute(&self.pool)
        .await?
        .rows_affected();
    Ok(result as usize)
}

  pub async fn check_email_sent(&self, raw_email: &str, item_guid: &str) -> Result<bool, sqlx::Error> {
    let email = clean_email(&raw_email).expect("Invalid email address");
    let sql = r#"
        SELECT EXISTS (
            SELECT 1 FROM emails_sent 
            WHERE email = ?1 AND item_guid = ?2 AND feed_url = ?3 AND success = false
        )
    "#;
    let exists = sqlx::query_scalar(sql)
        .bind(email)
        .bind(item_guid)
        .bind(&self.feed_url)
        .fetch_one(&self.pool)
        .await?;
    Ok(exists)
}

  pub async fn insert_feed_item(&self, item_guid: &str, skip: bool) -> Result<usize, sqlx::Error> {
    // Assuming you want to retain the current value of `all_emails_sent` or set a default if the row is new.
    // Adjust the `0` (false) for `all_emails_sent` if a different default is required for new rows.
    let sql = r#"
        INSERT INTO feed_items (feed_url, item_guid, all_emails_sent, skip) 
        VALUES (?1, ?2, 0, ?3)
        ON CONFLICT(item_guid) DO UPDATE SET 
        skip = EXCLUDED.skip
    "#;
    let result = sqlx::query(sql)
        .bind(&self.feed_url)
        .bind(item_guid)
        .bind(skip)
        .execute(&self.pool)
        .await?
        .rows_affected();
    Ok(result as usize)
  }

  pub async fn mark_all_emails_sent(&self, item_guid: &str) -> Result<usize, sqlx::Error> {
    let sql = r#"
        UPDATE feed_items SET all_emails_sent = 1 WHERE item_guid = ?1 AND feed_url = ?2
    "#;
    let result = sqlx::query(sql)
        .bind(item_guid)
        .bind(&self.feed_url)
        .execute(&self.pool)
        .await?
        .rows_affected();
    Ok(result as usize)
  }

  pub async fn get_feed_item(&self, item_guid: &str) -> Result<Option<FeedItem>, sqlx::Error> {
    let sql = r#"
      SELECT feed_url, item_guid, all_emails_sent, skip
      FROM feed_items 
      WHERE item_guid = ?1
      AND feed_url = ?2
    "#;
    let result = sqlx::query_as::<_, FeedItem>(sql)
        .bind(item_guid)
        .bind(&self.feed_url)
        .fetch_optional(&self.pool)
        .await?;
    Ok(result)
  }

}

