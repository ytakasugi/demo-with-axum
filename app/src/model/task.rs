use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow)]
pub struct Task {
    pub task_id: i32,
    pub user_id: i32,
    pub content: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub dead_line: NaiveDateTime,
    pub finished_flag: bool
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct SelectTask {
    pub task_id: i32,
    pub user_id: i32,
    pub content: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, sqlx::Type, sqlx::FromRow)]
pub struct NewTask {
    pub user_id: i32,
    pub content: String,
    pub dead_line: NaiveDateTime,
}