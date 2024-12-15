use askama::Template;
use super::menus_list;

#[derive(Template)]
#[template(path = "admin/table.html", escape = "none")]
pub struct ToolTemplate<'a> {
    pub name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
}

pub async fn menus() -> impl axum::response::IntoResponse {
    let name = "Admin Menus";
    let side_menus = menus_list::get_admin_menus();
    let template = ToolTemplate { name, menus: &side_menus };
    axum::response::Html(template.render().unwrap())
}
