use crate::{models::User, repositories::UserRepositoryTrait, services::UserServiceTrait};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    user_repository: Arc<dyn UserRepositoryTrait>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        self.user_repository
            .create(username, email, password_hash)
            .await
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.find_by_id(id).await
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.find_by_email(email).await
    }

    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.find_by_username(username).await
    }

    async fn update_user(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        bio: Option<&str>,
        image: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        self.user_repository
            .update(id, username, email, bio, image)
            .await
    }
}
