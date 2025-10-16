use axum::{Router, routing::get};

use crate::{handlers::health::health_check, state::AppState};

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}
