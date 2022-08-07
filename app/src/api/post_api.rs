use axum::{extract, response::{Result, IntoResponse}, Json};

use crate::util::db;
use crate::model::user::{User, NewUser};

pub async fn new_user(extract::Json(param): extract::Json<NewUser>) -> Result<impl IntoResponse> {
    let mut transaction = db::init()
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

    Ok(Json(format!("CREATE NEW USER: {}", parameter.user_name)))
}