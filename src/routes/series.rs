use crate::functions::{get_books_by_series, get_by_id};
use crate::structs::Library;

use rocket::{get, State};
use rocket_dyn_templates::Template;
use serde_json::{json, to_value};

#[get("/")]
pub fn series(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/series/list", context)
}

#[get("/<id>")]
pub fn series_page(id: u32, data: &State<Library>) -> Template {
    let series = &data.series;

    if let Some(s) = get_by_id(id, &series) {
        let books = get_books_by_series(s.id, &data.books);
        let author;
        if books.len() > 0 {
            author = get_by_id(books.get(0).unwrap().author_id, &data.authors).unwrap().clone();
            let context = json!({
                "series": s,
                "books": books,
                "author": author,
                "version": data.version,
                "last_update": data.last_update,
                "books_amount": books.len(),
            });
            return Template::render("pages/series/page", context);
        } else {
            let context = json!({
                "id": id,
                "version": data.version,
                "last_update": data.last_update,
                "message": "No author found"
            });
            Template::render("pages/error", context)
        }
    } else {
        let context = json!({
            "id": id,
            "version": data.version,
            "last_update": data.last_update,
            "message": format!("No such a series (id: {})", id)
        });

        Template::render("pages/error", context)
    }
}
