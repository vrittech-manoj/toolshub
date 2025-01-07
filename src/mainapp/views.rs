use askama::Template;
use axum::Extension;
use sea_orm::{EntityTrait, DatabaseConnection};
use crate::entities::post::Entity as Post;
// Define the Askama template
#[derive(Template)]
#[template(path = "index.html")] // Path relative to the `templates` folder
pub struct HomeTemplate<'a> {
    pub name: &'a str,
}

// Define the home function
pub async fn home(Extension(db): Extension<DatabaseConnection>) -> impl axum::response::IntoResponse {
    // Pass data to the template
    let post_entities = Post::find()
    .all(&db)
    .await
    .unwrap_or_else(|_| vec![]); // Handle database errors gracefully


    println!("this is some ... {:?}", post_entities);

    let template = HomeTemplate { name: "Axum User" };
    axum::response::Html(template.render().unwrap()) // Render and return HTML
}
