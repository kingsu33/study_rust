use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use std::net::SocketAddr;

fn send_response(status: StatusCode, body: &'static str) -> Response {
    let mut resp = Response::new(Body::from(body));
    *resp.status_mut() = status;

    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    resp.headers_mut()
        .insert(header::CONNECTION, HeaderValue::from_static("close"));

    resp
}

async fn health(req: Request) -> Response {
    // 요청 로그 (C처럼 method/path)
    println!("{} {}", req.method(), req.uri().path());
    send_response(StatusCode::OK, "{\"status\":\"ok\"}\n")
}

async fn not_found(req: Request) -> Response {
    println!("{} {}", req.method(), req.uri().path());
    send_response(
        StatusCode::NOT_FOUND,
        "{\"error\":{\"code\":\"NOT_FOUND\",\"message\":\"Route not found\"}}\n",
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .fallback(not_found);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Rust backend listening on 0.0.0.0:8080 (GET /health)");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
            println!("\nShutting down...");
        })
        .await
        .unwrap();
}
