use askama::Template;

// Define the Askama template
#[derive(Template)]
#[template(path = "tool.html")] // Path relative to the `templates` folder
pub struct aboutTemplate<'a> {
    pub name: &'a str,
}

// Define the home function
pub async fn about() -> impl axum::response::IntoResponse {
    // Pass data to the template
    let template = aboutTemplate { name: "Axum User" };
    axum::response::Html(template.render().unwrap()) // Render and return HTML
}
