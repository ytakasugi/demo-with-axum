use std::net::SocketAddr;

use axum::{
    routing::get,
    Router
};

mod util;
mod model;
mod api;

use api::get_api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = Router::new()
        .route("/users", get(get_api::get_user));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
