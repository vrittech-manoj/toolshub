use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

mod mainapp;
use mainapp::views::home;

#[tokio::main]
async fn main() {
    // Define the static file route
    let static_files = Router::new().nest_service(
        "/static", // URL prefix for static files
        ServeDir::new("static"), // Directory to serve files from
    );

    // Define the main app routes
    let app = Router::new()
        .merge(static_files) // Merge the static file router
        .route("/", get(home)); // Add the home route

    // Bind to localhost and start the server
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
