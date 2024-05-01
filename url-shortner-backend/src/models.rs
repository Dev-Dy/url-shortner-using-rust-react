use sqlx::{query, sqlite::SqlitePool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlMapping {
    pub short_code: String,
    pub long_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

impl UrlMapping {
    pub async fn create(pool: &SqlitePool, short_code: &str, long_url: &str) -> Result<(), sqlx::Error> {
        query!("INSERT INTO urls (short_code, long_url) VALUES (?, ?)", short_code, long_url)
            .execute(pool)
            .await?;
        Ok(())
    }
}
