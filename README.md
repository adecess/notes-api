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
├── notes_main/                 # Main Rust application
│   ├── src/
│   │   ├── main.rs            # Application entry point
│   │   ├── state.rs           # Shared application state
│   │   ├── handlers/          # HTTP request handlers
│   │   │   └── health.rs      # Health check endpoint
│   │   ├── models/            # Database entity models
│   │   │   └── user.rs        # User model
│   │   ├── repositories/      # Data access layer
│   │   │   └── user_repository.rs
│   │   └── schemas/           # API request/response schemas
│   │       └── user_schemas.rs
│   ├── migrations/            # Database migrations
│   │   └── 20250918111144_create_users_table.sql
│   └── Cargo.toml
├── initdb/                    # Database initialization
│   └── 00_from_database_url.sh
├── docker-compose.yml         # Development environment
└── README.md
```

## Getting Started

### Prerequisites

- Rust 1.70+ (2024 edition)
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
DATABASE_URL=postgres://notes_user:your_notes_password@localhost:5432/notes_db
POSTGRES_PASSWORD=your_postgres_password
```

### 3. Start Development Environment

```bash
# Start PostgreSQL database
docker-compose up -d

# Run the application
cd notes_main
cargo run
```

### 4. Test the API

```bash
# Health check
curl http://localhost:3000/health
```
