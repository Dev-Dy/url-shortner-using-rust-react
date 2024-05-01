use sha2::{Digest, Sha256};
use base64::encode_config;
use sqlx::{query_as, sqlite::SqlitePool};

pub fn shorten_url(long_url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(long_url);
    let hash = hasher.finalize();
    encode_config(&hash, base64::URL_SAFE_NO_PAD)
        .chars()
        .take(8)
        .collect::<String>()
}

pub async fn get_long_url(pool: &SqlitePool, short_code: &str) -> Result<Option<String>, sqlx::Error> {
    let record = query_as!(
        UrlMapping,
        "SELECT * FROM urls WHERE short_code = ?",
        short_code
    )
    .fetch_optional(pool)
    .await?;
    Ok(record.map(|r| r.long_url))
}
