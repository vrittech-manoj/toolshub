// $env:PATH = "C:\Program Files\PostgreSQL\16\bin;$env:PATH"
// cargo run

use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;


mod mainapp;
use mainapp::views::home;

mod tools;
use tools::views::{add_tools, tools};

mod admin;
use  admin::login::login;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    let db_connection = Arc::new(pool);
    println!("Database pool created successfully! {:?}",db_connection);
    // Define the main app routes
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static")) // Use nest_static instead
        .route("/", get(home))
        .route("/tools-hub/", get(tools))
        .route("/create-tools-hub/", get(add_tools))
        .route("/admin/",get(login));

    // Bind to localhost and start the server
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}