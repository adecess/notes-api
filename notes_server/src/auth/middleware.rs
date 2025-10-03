use crate::state::AppState;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{HeaderMap, StatusCode, request::Parts},
};
use services::User;

// For protected routes - requires valid JWT
pub struct RequireAuth(pub User);

impl<S> FromRequestParts<S> for RequireAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Extract Authorization header
        let headers = &parts.headers;
        let token = extract_token_from_headers(headers).ok_or(StatusCode::UNAUTHORIZED)?;

        // Validate JWT token
        let user_id = app_state
            .auth_service
            .validate_token(&token)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Get user from database
        let user = app_state
            .user_service
            .find_user_by_id(user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(RequireAuth(user))
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;

    auth_header
        .strip_prefix("Token ")
        .map(|token| token.to_string())
}
