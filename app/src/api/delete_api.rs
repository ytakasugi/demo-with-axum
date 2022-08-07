use axum::{extract, Json, response::{Result, IntoResponse}};

use crate::util::db;
use crate::model::user::User;

pub async fn delete_user(extract::Path(param): extract::Path<i32>) -> Result<impl IntoResponse> {
    let mut transaction = db::init()
        .await
        .begin()
        .await
        .unwrap();

    let user_id = param;

    let _ = sqlx::query_file_as!(
            User, 
            "sql/deleteUser.sql", 
            user_id
        )
        .fetch_one(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO CREATE NEW USER.")
        });

    /*let _ = sqlx::query(
            r#"
                DELETE FROM
                  USERS
                WHERE
                  USER_ID = $1
                  AND DELETE_FLAG = true
                RETURNING *
            "#
        )
        .bind(user_id)
        .execute(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO DELETE USER.")
        });*/

    /*let _ = sqlx::query_as::<_, User>(
            r#"
                DELETE FROM
                  USERS
                WHERE
                  USER_ID = $1
                  AND DELETE_FLAG = true
                RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO DELETE USER.")
        });*/

    transaction
        .commit()
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO COMMIT.")
        });

    Ok(Json(format!("DELETE USER: USER_ID = {}", user_id)))
} 