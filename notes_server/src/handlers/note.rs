use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use crate::{
    auth::middleware::RequireAuth,
    schemas::note_schemas::{CreateNoteRequest, NoteData, NoteResponse},
    state::AppState,
};

pub async fn create_note(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<Json<NoteResponse>, StatusCode> {
    // Validate input data
    payload
        .note
        .validate()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call note service
    let note = state
        .note_service
        .create_note(user.id, &payload.note.title, &payload.note.content)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Build response
    let note_data = NoteData::from_note(note);
    let response = NoteResponse { note: note_data };

    Ok(Json(response))
}
