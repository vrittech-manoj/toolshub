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
use tools::tools_routes;

mod admin;
use admin::{admin_routes,routes_names};


async fn not_found_handler() -> impl axum::response::IntoResponse {
    let route_names = routes_names::RouteNames::new();
    let mut response = String::from("<h1>Page Not Found</h1><p>Available Routes:</p><ul>");

    for (name, path) in route_names.routes {
        response.push_str(&format!("<li>{}: {}</li>", name, path));
    }

    response.push_str("</ul>");
    axum::response::Html(response)
}


#[tokio::main]
async fn main() {

    dotenv().ok();

    if let Some(admin_login_route) = routes_names::RouteNames::get("admin_login") {
        println!("Admin login route: {:?}",  admin_login_route);
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    let db_connection = Arc::new(pool);
    // println!("Database pool created successfully! {:?}",db_connection);
    // Define the main app routes
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static")) // Use nest_static instead
        .route("/", get(home))
        .merge(tools_routes())
        .merge(admin_routes())// Include admin routes
        .fallback(not_found_handler); // Attach the fallback handler
      

    // Bind to localhost and start the server
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}