use async_trait::async_trait;
use uuid::Uuid;

use crate::{User, models::Note};

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error>;

    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;

    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;

    async fn update_user(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        bio: Option<&str>,
        image: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;
}

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    InvalidPassword,
    UserAlreadyExists,
    DatabaseError(sqlx::Error),
    PasswordHashError,
    TokenGenerationError,
    TokenValidationError,
}

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::DatabaseError(err)
    }
}

#[async_trait]
pub trait AuthServiceTrait: Send + Sync {
    async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(User, String), AuthError>;

    async fn login_user(&self, email: &str, password: &str) -> Result<(User, String), AuthError>;

    async fn get_current_user(&self, user: User) -> Result<(User, String), AuthError>;

    async fn validate_token(&self, token: &str) -> Result<uuid::Uuid, AuthError>;
}

#[async_trait]
pub trait NoteServiceTrait: Send + Sync {
    async fn create_note(
        &self,
        user_id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<Note, sqlx::Error>;

    async fn find_note_by_id(&self, note_id: Uuid) -> Result<Option<Note>, sqlx::Error>;

    async fn find_notes_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, sqlx::Error>;

    async fn update_note(
        &self,
        note_id: Uuid,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<Option<Note>, sqlx::Error>;

    async fn delete_note(&self, note_id: Uuid) -> Result<Option<Note>, sqlx::Error>;
}
