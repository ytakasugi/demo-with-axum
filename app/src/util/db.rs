use dotenv::dotenv;
use sqlx::PgPool;

pub async fn get_connection() -> PgPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE URL MUST BE SET.");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO CONNECTION POOL.")
        });

    pool
}