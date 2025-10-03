use async_trait::async_trait;
use uuid::Uuid;

use crate::User;

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
