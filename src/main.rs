use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

mod mainapp;
use mainapp::views::home;

mod tools;
use tools::views::tools;

#[tokio::main]
async fn main() {
    // Define the main app routes
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static")) // Use nest_static instead
        .route("/", get(home))
        .route("/tools-hub/", get(tools));

    // Bind to localhost and start the server
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}