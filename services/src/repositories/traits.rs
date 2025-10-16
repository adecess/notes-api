use crate::models::{Note, User};
use async_trait::async_trait;
use sqlx::Error as SqlxError;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, SqlxError>;

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, SqlxError>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, SqlxError>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, SqlxError>;

    async fn update(
        &self,
        user_id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        bio: Option<&str>,
        image: Option<&str>,
    ) -> Result<Option<User>, SqlxError>;
}

#[async_trait]
pub trait NoteRepositoryTrait: Send + Sync {
    async fn create(&self, user_id: Uuid, title: &str, content: &str) -> Result<Note, SqlxError>;

    async fn find_note_by_id(
        &self,
        note_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Note>, SqlxError>;

    async fn find_all_notes(&self, user_id: Uuid) -> Result<Vec<Note>, SqlxError>;

    async fn update(
        &self,
        note_id: Uuid,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, SqlxError>;

    async fn delete(&self, note_id: Uuid) -> Result<Option<Note>, SqlxError>;
}
