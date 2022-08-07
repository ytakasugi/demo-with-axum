use axum::{extract, Json, response::{Result, IntoResponse}};
use chrono::Utc;

use crate::{util::db, model::user::{User, RegistUser}};

pub async fn refist_user_info(extract::Path(user_id): extract::Path<i32>, extract::Json(param): extract::Json<RegistUser>) -> Result<impl IntoResponse> {
    let mut transaction = db::init()
        .await
        .begin()
        .await
        .unwrap();

    let user_id = user_id;
    let now = Utc::now().naive_utc();

    let parameter = RegistUser {
        e_mail: param.e_mail,
        delete_flag: param.delete_flag,
    };

    if let Some(e_mail) = &parameter.e_mail {
        sqlx::query_file_as!(
            User,
            "sql/registUserEmail.sql",
            e_mail,
            now,
            user_id
        )
        .fetch_one(&mut transaction)
        .await
        .unwrap();
    };

    if let Some(delete_flag) = parameter.delete_flag {
        sqlx::query_file_as!(
                User, 
                "sql/logicalDeleteUser.sql", 
                delete_flag,
                now, 
                user_id
            )
            .fetch_one(&mut transaction)
            .await
            .unwrap_or_else(|_| {
                panic!("FAILED TO UPDATE USER.")
            });
    };
    

    transaction
        .commit()
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO COMMIT.")
        });

    Ok(Json(format!("UPDATE USER: USER_ID = {}", user_id)))
} 