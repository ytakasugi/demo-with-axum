use axum::{
    extract, 
    Json, 
    response::{
        Result, 
        IntoResponse
    }
};

use crate::{
    util::db, 
    model::user::{
        User, 
        RegistUser
    }
};

pub async fn regist_user(extract::Path(user_id): extract::Path<i32>, extract::Json(param): extract::Json<RegistUser>) -> Result<impl IntoResponse> {
    let mut transaction = db::get_connection_pool()
        .await
        .begin()
        .await
        .unwrap();

    let user_id = user_id;

    let parameter = RegistUser {
        e_mail: param.e_mail,
        delete_flag: param.delete_flag,
    };

    if let Some(e_mail) = &parameter.e_mail {
        sqlx::query_file_as!(
            User,
            "sql/registUserEmail.sql",
            e_mail,
            user_id
        )
        .fetch_one(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO UPDATE USER.")
        });
    };

    // delete_flag == trueだった場合
    if parameter.delete_flag {
        sqlx::query_file_as!(
            User, 
            "sql/logicalDeleteUser.sql", 
            user_id
        )
        .fetch_one(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO UPDATE USER.")
        });
    }
    
    transaction
        .commit()
        .await
        .unwrap_or_else(|_| {
            panic!("FAILED TO COMMIT.")
        });

    Ok(Json(format!("UPDATE USER: USER_ID = {}", user_id)))
} 