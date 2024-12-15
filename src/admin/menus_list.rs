#[derive(Debug, Clone)]
pub struct MenuItem {
    pub name: &'static str,
    pub path: &'static str,
}

pub fn get_admin_menus() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "Dashboard",
            path: "/admin/dashboard/",
        },
        MenuItem {
            name: "Tools",
            path: "/admin/dashboard/tools/",
        },
        MenuItem {
            name: "users",
            path: "/admin/dashboard/users/",
        },
    ]
}
