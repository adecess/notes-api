use axum::Router;
use std::env;

mod auth;
mod handlers;
mod routes;
mod schemas;
mod state;
use state::AppState;

use crate::routes::{
    auth_routes::auth_routes, health_routes::health_routes, note_routes::note_routes,
    user_routes::user_routes,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app_state = AppState::new(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database successfully!");

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/health", health_routes())
                .nest("/auth", auth_routes())
                .nest("/users", user_routes())
                .nest("/notes", note_routes()),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
