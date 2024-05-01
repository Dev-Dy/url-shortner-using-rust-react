use sqlx::{Pool, Sqlite};

pub async fn init_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = Pool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await?;
    Ok(pool)
}
