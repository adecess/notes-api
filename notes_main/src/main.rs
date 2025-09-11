use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_json() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello, world!",
    };

    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}
