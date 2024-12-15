use askama::Template;
use super::menus_list;

// Define the Askama template
#[derive(Template)]
#[template(path = "admin/index.html",escape = "none")] // Path relative to the `templates` folder
pub struct ToolTemplate<'a> {
    pub name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
}


// Define the home function
pub async fn dashboard() -> impl axum::response::IntoResponse {
    // Pass data to the template
    let name = "Admin Menus";
    let side_menus = menus_list::get_admin_menus();
    println!("{:?}",side_menus);
    let template = ToolTemplate { name, menus: &side_menus };

    axum::response::Html(template.render().unwrap()) // Render and return HTML
}
