use askama::Template;

// Define the Askama template
#[derive(Template)]
#[template(path = "tool.html")] // Path relative to the `templates` folder
pub struct ToolTemplate<'a> {
    pub name: &'a str,
}

// Define the home function
pub async fn tools() -> impl axum::response::IntoResponse {
    // Pass data to the template
    let template = ToolTemplate { name: "Axum User" };
    axum::response::Html(template.render().unwrap()) // Render and return HTML
}

pub async fn add_tools() -> impl axum::response::IntoResponse {
    // Pass data to the template
    let template = ToolTemplate { name: "Axum User" };
    axum::response::Html(template.render().unwrap()) // Render and return HTML
}