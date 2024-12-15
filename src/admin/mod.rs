pub mod views; // Declare the `views` module
pub mod login;
pub mod routes_names;

use axum::{
    routing::get,
    Router,
};

pub fn admin_routes() -> Router {
    Router::new()
        .route("/admin/login", get(login::login)) // Map the `/admin/login` route to `login` function
}
