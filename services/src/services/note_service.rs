use crate::{Note, repositories::traits::NoteRepositoryTrait, services::traits::NoteServiceTrait};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct NoteService {
    note_repository: Arc<dyn NoteRepositoryTrait>,
}

impl NoteService {
    pub fn new(note_repository: Arc<dyn NoteRepositoryTrait>) -> Self {
        Self { note_repository }
    }
}

#[async_trait]
impl NoteServiceTrait for NoteService {
    async fn create_note(
        &self,
        user_id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<Note, sqlx::Error> {
        self.note_repository.create(user_id, title, content).await
    }

    async fn find_note_by_id(
        &self,
        note_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Note>, sqlx::Error> {
        self.note_repository.find_note_by_id(note_id, user_id).await
    }

    async fn find_notes_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, sqlx::Error> {
        self.note_repository.find_all_notes(user_id).await
    }

    async fn update_note(
        &self,
        note_id: Uuid,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, sqlx::Error> {
        self.note_repository.update(note_id, title, content).await
    }

    async fn delete_note(&self, note_id: Uuid) -> Result<Option<Note>, sqlx::Error> {
        self.note_repository.delete(note_id).await
    }
}
