use dotenv::dotenv;
use sqlx::PgPool;

pub async fn get_connection() -> anyhow::Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE URL MUST BE SET.");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await;

    pool
}