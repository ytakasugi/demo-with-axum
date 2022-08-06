use axum::{Json, response::{Result, IntoResponse}};

use crate::util::db;
use crate::model::user::SelectUser;

pub async fn get_user() -> Result<impl IntoResponse> {
    let pool = db::init().await;

    let users = sqlx::query_file_as!(SelectUser, "sql/getUser.sql")
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(Json(users))
}
