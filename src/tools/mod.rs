pub mod views; // Declare the `views` module

use axum::{
    routing::get,
    Router,
};

pub fn tools_routes() -> Router {
    Router::new()
        .route("/tools/get-tools/", get(views::tools)) 
        .route("/tools/add-tools/",get(views::add_tools))
}
