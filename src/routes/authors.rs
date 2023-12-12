use crate::functions::{get_books_by_author, get_by_id, get_series_from_books};
use crate::structs::Library;

use rocket::{get, State};
use rocket_dyn_templates::Template;
use serde_json::{json, to_value};

#[get("/")]
pub fn authors(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/authors/list", context)
}

#[get("/<id>")]
pub fn author_page(id: u32, data: &State<Library>) -> Template {
    let authors = &data.authors;

    if let Some(author) = get_by_id(id, &authors) {
        let books = get_books_by_author(author.id, &data.books);
        let series = get_series_from_books(&data.series, &books);
        let context = json!({
            "author": author,
            "series": series,
            "books": books,
            "version": data.version,
            "last_update": data.last_update,
            "books_amount": books.len(),
            "series_amount": series.len()
        });
        Template::render("pages/authors/page", context)
    } else {
        let context = json!({
            "id": id,
            "version": data.version,
            "last_update": data.last_update,
            "message": format!("No such a author (id: {})", id)
        });

        Template::render("pages/error", context)
    }
}
