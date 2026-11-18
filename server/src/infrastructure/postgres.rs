use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_postgres_pool(postgres_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(postgres_url)
        .await
        .expect("failed to connect to database")
}
