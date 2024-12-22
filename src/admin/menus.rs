use askama::Template;
use axum::http::{method, Method};
use axum::http::uri::Scheme;
use super::menus_list;
use axum::{extract::Path, response::IntoResponse};
use super::form::Formdynamic;
use axum::extract::Form;
use super::form_fields;
use serde::Deserialize;
use std::collections::HashMap;

use super::store::store;

#[derive(Template)]
#[template(path = "admin/table.html", escape = "none")]
pub struct ToolTemplate<'a> {
    pub name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
}

pub async fn menus(Path(menu_name): Path<String>) -> impl axum::response::IntoResponse {
    let menu_name = menu_name;
    let side_menus = menus_list::get_admin_menus();
    let template = ToolTemplate { name:&menu_name, menus: &side_menus };
    axum::response::Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "admin/form.html", escape = "none")]
pub struct AddToolTemplate<'a> {
    pub menu_name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
    pub form: &'a Formdynamic,
}

#[derive(Deserialize, Debug)]
pub struct PostData {
    // Use a dynamic map to capture all fields and their values
    pub fields: HashMap<String, String>,
}

pub async fn add_menus(method: Method, Path(menu_name): Path<String>, Form(post_data): Form<HashMap<String, String>>,) -> impl axum::response::IntoResponse {
    println!("{:?}", method);

    match method {
        Method::POST => {
            println!("Received POST data: {:?}", post_data);
            println!("Received POST data:");
            for (key, value) in &post_data {
                println!("Field: {}, Value: {}", key, value);
            }
            let is_created = store().await;
            if is_created {
                axum::response::Html("Tool added successfully".to_string())
            } else {
                axum::response::Html("Failed to add tool".to_string())
            }
        }
        Method::GET => {
            let form_data = form_fields::return_tools_form().await;
            let side_menus = menus_list::get_admin_menus();
            let template = AddToolTemplate { 
                menu_name: &menu_name, 
                menus: &side_menus, 
                form: &form_data 
            };

            match template.render() {
                Ok(html) => axum::response::Html(html),
                Err(err) => {
                    eprintln!("Template rendering error: {}", err);
                    axum::response::Html("Internal Server Error".to_string())
                }
            }
        }
        _ => {
            println!("Unhandled HTTP method");
            axum::response::Html("Unhandled method".to_string())
        }
    }
}


