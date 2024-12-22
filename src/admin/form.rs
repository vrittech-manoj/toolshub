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
pub struct Formdynamic {
    pub action: String,
    pub method: String,
    pub fields: Vec<Field>,
    pub submit_label: String,
}

impl Formdynamic {
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