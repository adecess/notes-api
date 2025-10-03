use axum::extract::FromRef;
use services::{
    AuthService, AuthServiceTrait, UserRepository, UserService, UserServiceTrait,
    repositories::UserRepositoryTrait,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub user_service: Arc<dyn UserServiceTrait>,
    pub auth_service: Arc<dyn AuthServiceTrait>,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // Create the database connection pool
        let db = PgPool::connect(database_url).await?;

        // Run migrations automatically
        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository: Arc<dyn UserRepositoryTrait> =
            Arc::new(UserRepository::new(db.clone()));

        let user_service: Arc<dyn UserServiceTrait> = Arc::new(UserService::new(user_repository));

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let auth_service: Arc<dyn AuthServiceTrait> =
            Arc::new(AuthService::new(user_service.clone(), jwt_secret));

        Ok(Self {
            db,
            user_service,
            auth_service,
        })
    }
}
