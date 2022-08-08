use chrono::NaiveDateTime;

use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub e_mail: String,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub delete_flag: bool
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct SelectUser {
    pub user_id: i32,
    pub user_name: String,
    pub e_mail: String,
    //pub created: NaiveDateTime,
    //pub updated: Option<NaiveDateTime>,
    //pub delete_flag: bool
}

#[derive(Serialize, Deserialize, sqlx::Type, sqlx::FromRow)]
pub struct  NewUser {
    pub user_name: String,
    pub e_mail: String,
}

#[derive(Serialize, Deserialize, sqlx::Type, sqlx::FromRow)]
pub struct  RegistUser {
    pub e_mail: Option<String>,
    pub delete_flag: bool,
}