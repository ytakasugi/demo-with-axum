//use chrono::NaiveDateTime;

use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub e_mail: String,
    //pub created: NaiveDateTime,
    //pub updated: Option<NaiveDateTime>,
    //pub delete_flag: bool
}