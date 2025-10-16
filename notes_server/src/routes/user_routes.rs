use axum::{Router, routing::get};

use crate::{handlers::auth::current_user, state::AppState};

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/user", get(current_user))
}
