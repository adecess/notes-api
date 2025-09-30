# Notes API - Learning Axum & SQLx

A Rust web API built with [Axum](https://github.com/tokio-rs/axum) and [SQLx](https://github.com/launchbadge/sqlx) for learning modern Rust web development patterns.

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Handlers      │    │   Repositories  │    │     Models      │
│  (HTTP Layer)   │───▶│ (Data Access)   │───▶│ (Data Types)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   AppState      │    │   Database      │    │   Schemas       │
│ (Shared State)  │    │   (PostgreSQL)  │    │ (API Contracts) │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Project Structure

```
notes-api/
├── notes_server/               # Main Rust application
│   ├── src/
│   │   ├── main.rs            # Application entry point
│   │   ├── state.rs           # Shared application state
│   │   ├── handlers/          # HTTP request handlers
│   │   │   └── health.rs      # Health check endpoint
│   │   ├── auth/              # Auth (JWT, hashing, middleware)
│   │   │   ├── jwt.rs
│   │   │   ├── middleware.rs
│   │   │   └── password.rs
│   │   └── schemas/           # API request/response schemas
│   │       ├── auth_schemas.rs
│   │       └── user_schemas.rs
│   ├── migrations/            # Database migrations
│   │   └── 20250918111144_create_users_table.sql
│   └── Cargo.toml
├── services/                   # Business logic crate (models, repositories)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models/
│   │   │   └── user.rs
│   │   └── repositories/
│   │       └── user_repository.rs
│   └── Cargo.toml
├── initdb/                    # Database initialization
│   └── 00_from_database_url.sh
├── docker-compose.yml         # Development environment (db + app)
├── Dockerfile.notes-server    # Multi-stage build for the API binary
└── README.md
```

## Getting Started

### Prerequisites

- Rust 1.90+ (2024 edition)
- Docker & Docker Compose
- PostgreSQL client (optional)

### 1. Clone and Setup

```bash
git clone <your-repo>
cd notes-api
```

### 2. Environment Configuration

Create a `.env` file in the project root:

```env
## Local development (running `cargo run` on host)
DATABASE_URL=postgres://notes_user:your_notes_password@localhost:5432/notes_db

## Docker environment
DATABASE_URL_DOCKER=postgres://notes_user:your_notes_password@postgres:5432/notes_db

## Required by Postgres container
POSTGRES_PASSWORD=your_postgres_password

## JWT secret for auth
JWT_SECRET=your_jwt_secret_value
```

### 3. Start Development Environment

```bash
# Option A: Run with Docker (builds the app image and starts db + app)
docker-compose up -d --build

# Option B: Run locally (requires Rust toolchain and a running Postgres)
# Start only the database with Docker
docker-compose up -d postgres

# Then run the API on the host
cd notes_server
cargo run
```

### 4. Test the API

```bash
# Health check
curl http://localhost:3000/health

# Register a new user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"user": {"email": "alice@example.com", "password": "password123", "username": "alice"}}'

# Login and get a JWT
curl -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{"user": {"email": "alice@example.com", "password": "password123"}}'

# Get current user (replace TOKEN with the JWT from login)
curl http://localhost:3000/api/user \
  -H "Authorization: Bearer TOKEN"
```
