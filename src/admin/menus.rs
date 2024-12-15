use askama::Template;
use super::menus_list;
use axum::{extract::Path, response::IntoResponse};

#[derive(Template)]
#[template(path = "admin/table.html", escape = "none")]
pub struct ToolTemplate<'a> {
    pub name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
}

#[derive(Template)]
#[template(path = "admin/form.html", escape = "none")]
pub struct AddToolTemplate<'a> {
    pub name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
}

pub async fn menus(Path(menu_name): Path<String>) -> impl axum::response::IntoResponse {
    let menu_name = menu_name;
    let side_menus = menus_list::get_admin_menus();
    let template = ToolTemplate { name:&menu_name, menus: &side_menus };
    axum::response::Html(template.render().unwrap())
}

pub async fn add_menus(Path(menu_name): Path<String>) -> impl axum::response::IntoResponse {
    let menu_name = menu_name;
    let side_menus = menus_list::get_admin_menus();
    let template = AddToolTemplate { name:&menu_name, menus: &side_menus };
    axum::response::Html(template.render().unwrap())
}
