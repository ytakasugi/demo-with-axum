use axum::{extract, Json, response::{Result, IntoResponse}};
use chrono::Utc;

use crate::util::db;
use crate::model::user::{User/*, LogicalDeleteUser*/};

pub async fn logical_delete_user(extract::Path(param): extract::Path<i32>) -> Result<impl IntoResponse> {
    let mut transaction = db::init()
        .await
        .begin()
        .await
        .unwrap();

    /*let parameter = LogicalDeleteUser {
        user_id: param.user_id,
        delete_flag: param.delete_flag,
    };*/
    let user_id = param;
    let flag = true;
    let now = Utc::now().naive_utc();

    let query = sqlx::query_as::<_, User>(
        r#"
            UPDATE 
              USERS
            SET
              DELETE_FLAG = $1
              , UPDATED = $2
            WHERE
              USER_ID = $3
            RETURNING *
        "#
    )
    .bind(flag)
    .bind(now)
    .bind(user_id)
    .fetch_one(&mut transaction)
    .await
    .unwrap_or_else(|_| {
        panic!("FAILED TO UPDATE USER.")
    });

    transaction
        .commit()
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO COMMIT.")
        });

    Ok(Json(format!("UPDATE USER: {}", query.user_id)))
} 