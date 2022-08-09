use axum::{extract, response::{Result, IntoResponse}, Json};

use crate::util::db;
use crate::model::user::{User, NewUser};
use crate::model::task::{Task, NewTask};

pub async fn new_user(extract::Json(param): extract::Json<NewUser>) -> Result<impl IntoResponse> {
    let mut transaction = db::get_connection()
        .await
        .begin()
        .await
        .unwrap();

    let parameter = NewUser {
        user_name: param.user_name.to_string(),
        e_mail: param.e_mail.to_string(),
    };

    let _ = sqlx::query_file_as!(
            User, 
            "sql/insertUser.sql", 
            &parameter.user_name, 
            &parameter.e_mail
        )
        .fetch_one(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO CREATE NEW USER.")
        });

    transaction
        .commit()
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO COMMIT.")
        });

    Ok(Json(format!("CREATE NEW USER: USER_NAME = {}", parameter.user_name)))
}

pub async fn new_task(extract::Json(param): extract::Json<NewTask>) -> Result<impl IntoResponse> {
    let mut transaction = db::get_connection()
        .await
        .begin()
        .await
        .unwrap();

    let parameter = NewTask {
        user_id: param.user_id,
        content: param.content.to_string(),
        dead_line: param.dead_line
    };

    let query = sqlx::query_file_as!(
        Task, 
        "sql/insertTask.sql", 
        parameter.user_id,
        &parameter.content, 
        parameter.dead_line
    )
    .fetch_one(&mut transaction)
    .await
    .unwrap_or_else(|_| {
        panic!("FAILED TO CREATE NEW TASK.")
    });

    transaction
        .commit()
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO COMMIT.")
        });

        Ok(Json(format!("CREATE NEW TASK: TASK_ID = {}", query.task_id)))
}