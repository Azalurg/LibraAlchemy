use serde_json;
use handlebars::Handlebars;
use std::fs::{self, File, read_to_string};
use std::io::Write;

use crate::structs::Library;

pub fn build(json_file_path: &str, output_path: &str) {
    // Read the template files
    let template_main = fs::read_to_string("assets/templates/main.hbs").expect("Failed to read template file");
    let template_author = fs::read_to_string("assets/templates/author.hbs").expect("Failed to read template file");
    let template_album = fs::read_to_string("assets/templates/album.hbs").expect("Failed to read template file");
    let template_book = fs::read_to_string("assets/templates/book.hbs").expect("Failed to read template file");

    // Create a Handlebars instance
    let mut handlebars = Handlebars::new();

    // Register the template
    handlebars.register_template_string("main", template_main).expect("Failed to register template");
    handlebars.register_template_string("author", template_author).expect("Failed to register template");
    handlebars.register_template_string("album", template_album).expect("Failed to register template");
    handlebars.register_template_string("book", template_book).expect("Failed to register template");

    // Read JSON data from file
    let json_data = read_to_string(json_file_path).expect("Failed to read JSON file");

    // Deserialize the JSON data into your structured data
    let lib: Library = serde_json::from_str(&json_data)
        .expect("Failed to deserialize JSON data");

    // Serialize the data to JSON
    let json_value = serde_json::to_value(&lib).expect("Failed to serialize data to JSON");

    // Render the template with the JSON data
    let rendered_html = handlebars.render("main", &json_value).expect("Failed to render template");

    // Write the rendered HTML to the output file
    let mut html_file = File::create(output_path).expect("Failed to create HTML file");
    html_file.write_all(rendered_html.as_bytes()).expect("Failed to write HTML data to file");

    println!("HTML file generated: {}", output_path);
}