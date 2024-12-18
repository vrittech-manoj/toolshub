use askama::Template;
use axum::http::{method, Method};
use axum::http::uri::Scheme;
use super::menus_list;
use axum::{extract::Path, response::IntoResponse};
use super::form::Form;
use super::form_fields;

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
    
pub struct Person {
    name: String,
    age: u32,
}

#[derive(Template)]
#[template(path = "admin/form.html", escape = "none")]
pub struct AddToolTemplate<'a> {
    pub menu_name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
    pub form: &'a Form,
}

pub async fn add_menus(method:Method,Path(menu_name): Path<String>) -> impl axum::response::IntoResponse {

    // println!("{:?}",form_fields::return_tools_form().await);
    println!("{:?}",method);
    let form_data = form_fields::return_tools_form().await;
    let menu_name = menu_name;
    let side_menus = menus_list::get_admin_menus();
    let template = AddToolTemplate { menu_name:&menu_name, menus: &side_menus,form: &form_data};
    axum::response::Html(template.render().unwrap())

}


