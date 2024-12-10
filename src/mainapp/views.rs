use askama::Template;

// Define the Askama template
#[derive(Template)]
#[template(path = "index.html")] // Path relative to the `templates` folder
pub struct HomeTemplate<'a> {
    pub name: &'a str,
}

// Define the home function
pub async fn home() -> impl axum::response::IntoResponse {
    // Pass data to the template
    let template = HomeTemplate { name: "Axum User" };
    axum::response::Html(template.render().unwrap()) // Render and return HTML
}
