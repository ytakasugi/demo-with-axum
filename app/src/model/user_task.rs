use chrono::NaiveDateTime;

use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct SelectUserTask {
    pub task_id: i32,
    pub user_name: String,
    pub content: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub dead_line: NaiveDateTime,
}