use handlebars::Handlebars;
use serde_json;
use std::fs::{read_to_string, File};
use std::io::Write;

use crate::structs::Library;
use crate::templates::get_templates;

pub fn build(json_path: &str, output_path: &str) {
    println!("Building html file in: {}", output_path);
    // Read the template files (development only)
    let templates = get_templates();

    // Create a Handlebars instance
    let mut handlebars = Handlebars::new();

    // Register the template
    handlebars
        .register_template_string("main", templates.main)
        .expect("Failed to register template");
    handlebars
        .register_template_string("author", templates.author)
        .expect("Failed to register template");
    handlebars
        .register_template_string("series", templates.series)
        .expect("Failed to register template");
    handlebars
        .register_template_string("book", templates.book)
        .expect("Failed to register template");
    handlebars
        .register_template_string("style", templates.style)
        .expect("Failed to register template");

    // Read JSON data from file
    let json_data = read_to_string(json_path).expect("Failed to read JSON file");

    // Deserialize the JSON data into your structured data
    let lib: Library = serde_json::from_str(&json_data).expect("Failed to deserialize JSON data");

    // Serialize the data to JSON
    let json_value = serde_json::to_value(&lib).expect("Failed to serialize data to JSON");

    // Render the template with the JSON data
    let rendered_html = handlebars.render("main", &json_value).expect("Failed to render template");

    // Write the rendered HTML to the output file
    let mut html_file = File::create(output_path).expect("Failed to create HTML file");
    html_file
        .write_all(rendered_html.as_bytes())
        .expect("Failed to write HTML data to file");

    println!("Building complied");
}
