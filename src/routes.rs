use axum::{
    routing::get,
    Router,Extension,
};
use tower_http::services::ServeDir;

use crate::mainapp::views::home;
use crate::tools::tools_routes;
use crate::admin::{admin_routes, routes_names};
use crate::database_connection::{DbPool,create_db_pool};

pub async fn not_found_handler() -> impl axum::response::IntoResponse {
    let route_names = routes_names::RouteNames::new();
    let mut response = String::from("<h1>Page Not Found</h1><p>Available Routes:</p><ul>");

    for (name, path) in route_names.routes {
        response.push_str(&format!("<li>{}: {}</li>", name, path));
    }

    response.push_str("</ul>");
    axum::response::Html(response)
}

// Function to configure the main router
pub fn configure_app_router() -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(home))
        .merge(tools_routes())
        .merge(admin_routes())
        .fallback(not_found_handler)
        .layer(Extension(create_db_pool)) // Add database pool as an extension
}
