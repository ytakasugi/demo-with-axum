use axum::{Json, response::{Result, IntoResponse}};

use crate::util::db;
use crate::model::user::User;

pub async fn get_user() -> Result<impl IntoResponse> {
    let pool = db::get_connection().await.unwrap();

    let users = sqlx::query_file_as!(User, "sql/getUser.sql")
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(Json(users))
}
