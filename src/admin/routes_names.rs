use std::collections::HashMap;

pub struct RouteNames {
    pub routes: HashMap<&'static str, &'static str>,
}

impl RouteNames {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        routes.insert("admin_login", "/admin/login/");
        routes.insert("admin_dashboard", "/admin/dashboard/");
        routes.insert("get-tools", "/tools/get-tools");
        routes.insert("admin.get_tools","/admin/dashboard/:menu_name/");
        routes.insert("admin.add_tools","/admin/dashboard/add/:menu_name/");
        Self { routes }
    }

    pub fn get(name: &str) -> Option<&'static str> {
        let routes = RouteNames::new();
        routes.routes.get(name).copied()
    }
}
