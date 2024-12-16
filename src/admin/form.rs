// FormField Struct and Configuration
#[derive(Debug)]
pub struct FormField {
    pub label: String,
    pub name: String,
    pub field_type: String, // e.g., "text", "email", "file", "select", "radio"
    pub placeholder: Option<String>,
    pub choices: Option<Vec<(String, String)>>, // For "select" or "radio"
}

pub fn get_form_config() -> Vec<FormField> {
    vec![
        FormField {
            label: "Project Name".to_string(),
            name: "project_name".to_string(),
            field_type: "text".to_string(),
            placeholder: Some("Enter the project name".to_string()),
            choices: None,
        },
        FormField {
            label: "Title".to_string(),
            name: "title".to_string(),
            field_type: "text".to_string(),
            placeholder: Some("Enter the title".to_string()),
            choices: None,
        },
        FormField {
            label: "Email".to_string(),
            name: "email".to_string(),
            field_type: "email".to_string(),
            placeholder: Some("name@example.com".to_string()),
            choices: None,
        },
        FormField {
            label: "Options".to_string(),
            name: "options".to_string(),
            field_type: "select".to_string(),
            placeholder: None,
            choices: Some(vec![
                ("1".to_string(), "Option 1".to_string()),
                ("2".to_string(), "Option 2".to_string()),
                ("3".to_string(), "Option 3".to_string()),
            ]),
        },
        FormField {
            label: "Gender".to_string(),
            name: "gender".to_string(),
            field_type: "radio".to_string(),
            placeholder: None,
            choices: Some(vec![
                ("male".to_string(), "Male".to_string()),
                ("female".to_string(), "Female".to_string()),
            ]),
        },
    ]
}