use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::middleware::RequireAuth,
    schemas::note_schemas::{CreateNoteRequest, NoteData, NoteListResponse, NoteResponse},
    state::AppState,
};

pub async fn create_note(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<Json<NoteResponse>, StatusCode> {
    payload
        .note
        .validate()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let note = state
        .note_service
        .create_note(user.id, &payload.note.title, &payload.note.content)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let note_data = NoteData::from_note(note);
    let response = NoteResponse { note: note_data };

    Ok(Json(response))
}

pub async fn find_note_by_id(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<NoteResponse>, StatusCode> {
    let Some(note) = state
        .note_service
        .find_note_by_id(note_id, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let note_data = NoteData::from_note(note);
    let response = NoteResponse { note: note_data };

    Ok(Json(response))
}

pub async fn find_all_notes(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Json<NoteListResponse>, StatusCode> {
    let notes = state
        .note_service
        .find_notes_by_user_id(user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let note_list_response = NoteListResponse::from_notes(notes);

    Ok(Json(note_list_response))
}
