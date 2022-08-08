use axum::{Json, response::{Result, IntoResponse}};

use crate::util::db;
use crate::model::user::SelectUser;
use crate::model::user_task::SelectUserTask;

pub async fn get_user() -> Result<impl IntoResponse> {
    let pool = db::init().await;

    let users = sqlx::query_file_as!(
            SelectUser, 
            "sql/getUser.sql"
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(Json(users))
}

pub async fn get_user_task() -> Result<impl IntoResponse> {
    let pool = db::init().await;

    let user_task = sqlx::query_file_as!(
            SelectUserTask, 
            "sql/getUserTask.sql"
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(Json(user_task))
}
