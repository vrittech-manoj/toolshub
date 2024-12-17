use askama::Template;
use super::menus_list;
use axum::{extract::Path, response::IntoResponse};

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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Email,
    File,
    Select,
    Radio,
    Number,
}

impl PartialEq<&str> for FieldType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            FieldType::Text => *other == "Text",
            FieldType::Email => *other == "Email",
            FieldType::File => *other == "File",
            FieldType::Select => *other == "Select",
            FieldType::Radio => *other == "Radio",
            FieldType::Number => *other == "Number",
        }
    }
}

use std::fmt;

impl fmt::Display for FieldType { //this is used to display in template
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let field_type_str = match self {
            FieldType::Text => "Text",
            FieldType::Email => "Email",
            FieldType::File => "File",
            FieldType::Select => "Select",
            FieldType::Radio => "Radio",
            FieldType::Number => "Number",
        };
        write!(f, "{}", field_type_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub label: String,
    pub required: bool,
    pub placeholder: Option<String>,
    pub choices: Option<Vec<(String, String)>>, // For select and radio fields
    pub value: Option<String>,
    pub css_class: Option<String>,
}

impl Field {
    pub fn display_placeholder(&self) -> &str {
        self.placeholder.as_deref().unwrap_or("Default Placeholder")
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub action: String,
    pub method: String,
    pub fields: Vec<Field>,
    pub submit_label: String,
}

impl Form {
    pub fn new(action: &str) -> Self {
        Self {
            action: action.to_string(),
            method: "POST".to_string(),
            fields: Vec::new(),
            submit_label: "Save".to_string(),
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}

// Helper functions to create common field types
pub fn text_field(name: &str, label: &str) -> Field {
    Field {
        name: name.to_string(),
        field_type: FieldType::Text,
        label: label.to_string(),
        required: true,
        placeholder: Some(format!("Enter {}", label)),
        choices: None,
        value: None,
        css_class: Some("form-control".to_string()),
    }
}

pub fn email_field(name: &str, label: &str) -> Field {
    Field {
        name: name.to_string(),
        field_type: FieldType::Email,
        label: label.to_string(),
        required: true,
        placeholder: Some("name@example.com".to_string()),
        choices: None,
        value: None,
        css_class: Some("form-control".to_string()),
    }
}

pub fn select_field(name: &str, label: &str, choices: Vec<(String, String)>) -> Field {
    Field {
        name: name.to_string(),
        field_type: FieldType::Select,
        label: label.to_string(),
        required: true,
        placeholder: None,
        choices: Some(choices),
        value: None,
        css_class: Some("form-select".to_string()),
    }
}
    
pub struct Person {
    name: String,
    age: u32,
}

#[derive(Template)]
#[template(path = "admin/form.html", escape = "none")]
pub struct AddToolTemplate<'a> {
    pub name: &'a str,
    pub menus: &'a Vec<menus_list::MenuItem>,
    pub peoples:&'a Vec<Person>,
    pub form: &'a Form,
}


pub async fn add_menus(Path(menu_name): Path<String>) -> impl axum::response::IntoResponse {
    let mut people: Vec<Person> = Vec::new();
    let mut form = Form::new("/admin/dashboard/add/tools/");
    
    // Add fields to match your HTML
    form.add_field(text_field("projectName", "Project Name"));
    form.add_field(text_field("title", "Title"));
    form.add_field(email_field("email", "Email"));
    
    // Add file input
    form.add_field(Field {
        name: "formFile".to_string(),
        field_type: FieldType::File,
        label: "Default file input example".to_string(),
        required: false,
        placeholder: None,
        choices: None,
        value: None,
        css_class: Some("form-control bg-dark".to_string()),
    });
    
    // Add select field
    let select_choices = vec![
        ("1".to_string(), "One".to_string()),
        ("2".to_string(), "Two".to_string()),
        ("3".to_string(), "Three".to_string()),
    ];
    form.add_field(select_field("select", "Select Menu", select_choices));
    
    // Add radio buttons
    let radio_choices = vec![
        ("1".to_string(), "Default radio".to_string()),
        ("2".to_string(), "Default checked radio".to_string()),
    ];
    let mut radio_field = Field {
        name: "flexRadioDefault".to_string(),
        field_type: FieldType::Radio,
        label: "Radio Options".to_string(),
        required: false,
        placeholder: None,
        choices: Some(radio_choices),
        value: Some("2".to_string()), // This will make the second option checked
        css_class: None,
    };
    form.add_field(radio_field);
    // Add elements to the vector
    people.push(Person {
        name: String::from("Alice"),
        age: 30,
    });
    println!("{:?}",form);
    let menu_name = menu_name;
    let side_menus = menus_list::get_admin_menus();
    let template = AddToolTemplate { name:&menu_name, menus: &side_menus,peoples:&people,form: &form,};
    axum::response::Html(template.render().unwrap())
}


