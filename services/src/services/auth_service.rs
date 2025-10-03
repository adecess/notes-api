use crate::{
    models::User,
    services::{
        UserServiceTrait,
        traits::{AuthError, AuthServiceTrait},
    },
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthService {
    user_service: Arc<dyn UserServiceTrait>,
    jwt_secret: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

impl AuthService {
    pub fn new(user_service: Arc<dyn UserServiceTrait>, jwt_secret: String) -> Self {
        Self {
            user_service,
            jwt_secret,
        }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(User, String), AuthError> {
        // Check if user already exists
        if self.user_service.find_user_by_email(email).await?.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        if self
            .user_service
            .find_user_by_username(username)
            .await?
            .is_some()
        {
            return Err(AuthError::UserAlreadyExists);
        }

        // Hash the password
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST + 2)
            .map_err(|_| AuthError::PasswordHashError)?;

        // Create user
        let user = self
            .user_service
            .create_user(username, email, &password_hash)
            .await?;

        // Generate JWT token
        let token = self.generate_token(&user.id)?;

        Ok((user, token))
    }

    async fn login_user(&self, email: &str, password: &str) -> Result<(User, String), AuthError> {
        // Find user by email
        let user = self
            .user_service
            .find_user_by_email(email)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        // Verify password
        let password_valid = bcrypt::verify(password, &user.password_hash)
            .map_err(|_| AuthError::PasswordHashError)?;

        if !password_valid {
            return Err(AuthError::InvalidPassword);
        }

        // Generate JWT token
        let token = self.generate_token(&user.id)?;

        Ok((user, token))
    }

    async fn get_current_user(&self, user: User) -> Result<(User, String), AuthError> {
        let token = self.generate_token(&user.id)?;
        Ok((user, token))
    }

    async fn validate_token(&self, token: &str) -> Result<Uuid, AuthError> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::TokenValidationError)?;

        let user_id =
            Uuid::parse_str(&data.claims.sub).map_err(|_| AuthError::TokenValidationError)?;

        Ok(user_id)
    }
}

impl AuthService {
    fn generate_token(&self, user_id: &uuid::Uuid) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = (now + Duration::hours(1)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AuthError::TokenGenerationError)?;

        Ok(token)
    }
}
