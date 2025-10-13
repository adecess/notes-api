use serde::{Deserialize, Serialize};
use services::Note;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub note: CreateNoteData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteData {
    #[validate(length(max = 50, message = "Title cannot exceed 50 characters"))]
    pub title: String,

    #[validate(length(max = 500, message = "Content cannot exceed 500 characters"))]
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub note: NoteData,
}

#[derive(Debug, Serialize)]
pub struct NoteData {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

impl NoteData {
    pub fn from_note(note: Note) -> Self {
        Self {
            user_id: note.user_id,
            title: note.title,
            content: note.content,
        }
    }
}
