use axum::Router;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};

use tokio::signal;

mod mainapp;
mod tools;
mod admin;
mod routes;
mod database_connection;
use std::env;


#[tokio::main]
async fn main() {
    color_eyre::install();
    dotenv().ok(); // Load environment variables
    // Configure the application router
    let app = routes::configure_app_router().await;

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    println!("Shutting down gracefully...");
}
