use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::note::{create_note, find_note_by_id},
    state::AppState,
};

pub fn note_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_note))
        .route("/{id}", get(find_note_by_id))
    // .route("/me", get(get_current_user_notes))
}
