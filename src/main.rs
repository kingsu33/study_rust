mod api;
mod app;
mod domain;
mod infra;
mod repository;
mod service;

use  app::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.expect("failed to bind");

    println!("listening on http://localhost:3000");

    axum::serve(listener, app).await.expect("server error");
}