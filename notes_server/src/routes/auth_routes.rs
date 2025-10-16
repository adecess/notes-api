use axum::{Router, routing::post};

use crate::{
    handlers::auth::{login, register},
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
