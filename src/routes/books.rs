use crate::functions::get_by_id;
use crate::structs::Library;

use rocket::{get, State};
use rocket_dyn_templates::Template;
use serde_json::{json, to_value};

#[get("/")]
pub fn books(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/books/list", context)
}

#[get("/<id>")]
pub fn book_page(id: u32, data: &State<Library>) -> Template {
    let books = &data.books;

    if let Some(book) = get_by_id(id, &books) {
        let context = json!({
            "book": book,
            "version": data.version,
            "last_update": data.last_update
        });
        Template::render("pages/books/page", context)
    } else {
        let context = json!({
            "id": id,
            "version": data.version,
            "last_update": data.last_update,
            "message": format!("No such a book (id: {})", id)
        });

        Template::render("pages/error", context)
    }
}
