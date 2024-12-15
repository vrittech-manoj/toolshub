use askama::Template;

// Define the Askama template
#[derive(Template)]
#[template(path = "admin/signin.html",escape = "none")] // Path relative to the `templates` folder
pub struct ToolTemplate<'a> {
    pub name: &'a str,
}

// Define the home function
pub async fn dashboard() -> impl axum::response::IntoResponse {
    // Pass data to the template
    let template = ToolTemplate { name: "Axum User" };
    axum::response::Html(template.render().unwrap()) // Render and return HTML
}
