use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("DB connection failed")
}
