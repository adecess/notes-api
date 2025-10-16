use axum::{Router, routing::post};

use crate::{handlers::note::create_note, state::AppState};

pub fn note_routes() -> Router<AppState> {
    Router::new().route("/", post(create_note))
}
