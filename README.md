# Rust Axum Web Framework Tutorial

A practical guide to building web applications with Axum, a modern Rust web framework focused on ergonomics and modularity.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Project Setup](#project-setup)
- [Basic Server](#basic-server)
- [Route Handlers](#route-handlers)
- [Working with JSON](#working-with-json)
- [Path Parameters](#path-parameters)
- [Query Parameters](#query-parameters)
- [Middleware](#middleware)
- [Error Handling](#error-handling)
- [Database Integration](#database-integration)

## Prerequisites

Before starting, ensure you have:
- Rust installed (1.75.0 or later)
- Cargo package manager
- Basic understanding of Rust syntax and concepts

## Project Setup

Create a new Rust project:

```bash
cargo new axum_tutorial
cd axum_tutorial
```

Add dependencies to `Cargo.toml`:

```toml
[package]
name = "axum_tutorial"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower = "0.4"
tower-http = { version = "0.5", features = ["trace"] }
```

## Basic Server

Here's a basic server implementation:

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Initialize router
    let app = Router::new()
        .route("/", get(root_handler));
    
    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, Axum!"
}
```

## Route Handlers

Axum supports various types of route handlers:

```rust
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

// Define data structures
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ];
    Json(users)
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 3, // In a real app, generate or get from database
        name: payload.name,
    };
    Json(user)
}

// Add routes to router
let app = Router::new()
    .route("/users", get(get_users))
    .route("/users", post(create_user));
```

## Working with JSON

Example of handling JSON requests and responses:

```rust
use axum::{
    extract::Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // In a real app, validate credentials
    let token = format!("token-{}", payload.username);
    
    Json(LoginResponse { token })
}
```

## Path Parameters

Handling URL parameters:

```rust
use axum::{
    extract::Path,
    response::IntoResponse,
};

async fn get_user_by_id(Path(user_id): Path<u32>) -> impl IntoResponse {
    format!("User ID: {}", user_id)
}

// Add route
let app = Router::new()
    .route("/users/:id", get(get_user_by_id));
```

## Query Parameters

Working with query strings:

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_items(Query(params): Query<Params>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("Page: {}, Limit: {}", page, limit)
}

// Add route
let app = Router::new()
    .route("/items", get(list_items));
```

## Middleware

Adding middleware for logging and authentication:

```rust
use tower_http::trace::TraceLayer;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::http::Request;

async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>
) -> Response {
    // Check authorization header
    if let Some(auth_header) = request.headers().get("Authorization") {
        // Validate token in a real app
        return next.run(request).await;
    }
    
    Response::builder()
        .status(401)
        .body("Unauthorized".into())
        .unwrap()
}

// Add middleware to router
let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(TraceLayer::new_for_http()) // Logging
    .layer(middleware::from_fn(auth_middleware)); // Authentication
```

## Error Handling

Implementing custom error handling:

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

#[derive(Debug)]
enum AppError {
    NotFound,
    InvalidInput,
    DatabaseError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::InvalidInput => (StatusCode::BAD_REQUEST, "Invalid input"),
            AppError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };
        
        (status, message).into_response()
    }
}

async fn handler_with_error() -> Result<String, AppError> {
    Err(AppError::NotFound)
}
```

## Database Integration

Example using SQLx with PostgreSQL:

```rust
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn create_db_pool() -> PgPool {
    PgPool::connect("postgres://user:password@localhost/db_name")
        .await
        .unwrap()
}

// Use with router
let db_pool = create_db_pool().await;
let app_state = AppState { db: db_pool };

let app = Router::new()
    .route("/users", get(get_users))
    .with_state(app_state);
```

## Running the Server

To run your Axum application:

```bash
cargo run
```

Visit `http://localhost:3000` in your browser or use tools like cURL or Postman to test your endpoints.

## Best Practices

1. **Organization**: Group related routes and handlers in separate modules
2. **Error Handling**: Implement custom error types and proper error handling
3. **Configuration**: Use environment variables for configuration
4. **Testing**: Write unit and integration tests for your handlers
5. **Documentation**: Add documentation comments to your code
6. **Logging**: Implement proper logging for production environments
7. **Security**: Use middleware for authentication and input validation

### migrations--sea--orm

cargo install sea-orm-cli
sea-orm-cli migrate init
sea-orm-cli migrate generate <migration_name>
sea-orm-cli migrate up
sea-orm-cli generate entity -u postgres://postgres:md@localhost/toolshub -o src/entities     

