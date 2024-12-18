use super::form::{FieldType,Field,Form,text_field,email_field,select_field};

pub async fn return_form()-> Form {

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
    let radio_field = Field {
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
    form

}
