pub mod views; // Declare the `views` module
pub mod login;
pub mod dashboard;
pub mod routes_names;
pub mod menus;
pub mod menus_list;


use axum::{
    routing::get,
    Router,
};

pub fn admin_routes() -> Router {
    Router::new()
        .route("/admin/login/", get(login::login)) // Map the `/admin/login` route to `login` function
        .route("/admin/dashboard/", get(dashboard::dashboard))
        .route("/admin/dashboard/:menu_name/", get(menus::menus))
        .route("/admin/dashboard/add/:menu_name/", get(menus::add_menus))
}
