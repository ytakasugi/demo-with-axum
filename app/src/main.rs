use std::net::SocketAddr;

use axum::{
    routing::{get, post, put, delete},
    Router
};

mod util;
mod model;
mod api;

use api::{get_api, post_api, put_api, delete_api};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = Router::new()
        .route("/users", get(get_api::get_user))
        .route("/users", post(post_api::new_user))
        .route("/users/:user_id", put(put_api::regist_user))
        .route("/users/:user_id", delete(delete_api::delete_user))
        .route("/users/tasks", get(get_api::get_user_task))
        .route("/tasks", post(post_api::new_task));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}