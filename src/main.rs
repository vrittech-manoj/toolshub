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
mod tools;
mod admin;
mod routes;
mod database_connection;


#[tokio::main]
async fn main() {

    dotenv().ok();


    let app = routes::configure_app_router();

    // Bind to localhost and start the server
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}