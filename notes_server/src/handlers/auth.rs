use crate::{auth::middleware::RequireAuth, schemas::auth_schemas::*, state::AppState};
use axum::{Json, extract::State, http::StatusCode};
use services::services::traits::AuthError;
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Validate input data
    payload
        .user
        .validate()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call auth service
    let (user, token) = state
        .auth_service
        .register_user(
            &payload.user.username,
            &payload.user.email,
            &payload.user.password,
        )
        .await
        .map_err(|err| match err {
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::PasswordHashError | AuthError::TokenGenerationError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    // Build response
    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Validate input
    payload
        .user
        .validate()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call auth service
    let (user, token) = state
        .auth_service
        .login_user(&payload.user.email, &payload.user.password)
        .await
        .map_err(|err| match err {
            AuthError::UserNotFound | AuthError::InvalidPassword => StatusCode::UNAUTHORIZED,
            AuthError::PasswordHashError | AuthError::TokenGenerationError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    // Build response
    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}

pub async fn current_user(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Call auth service
    let (user, token) =
        state
            .auth_service
            .get_current_user(user)
            .await
            .map_err(|err| match err {
                AuthError::TokenGenerationError => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })?;

    // Build response
    let user_data = UserData::from_user_with_token(user, token);
    let response = UserResponse { user: user_data };

    Ok(Json(response))
}
