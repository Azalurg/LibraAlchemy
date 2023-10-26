use std::collections::HashSet;

use self::handlebars::{Handlebars, JsonRender};
use crate::structs::{Book, Library, Series, ID};
use rocket::{figment::value::Value, get, http::ext::IntoCollection, State};
use rocket_dyn_templates::{context, handlebars, Template};
use serde_json::{json, to_value};

#[get("/")]
pub fn index(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/home", context)
}

#[get("/static")]
pub fn statics(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("static", context)
}

#[get("/books")]
pub fn books(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/books/list", context)
}

#[get("/authors")]
pub fn authors(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/authors/list", context)
}

#[get("/series")]
pub fn series(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render("pages/series/list", context)
}

#[get("/books/<id>")]
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

#[get("/author/<id>")]
pub fn author_page(id: u32, data: &State<Library>) -> Template {
    let authors = &data.authors;

    if let Some(author) = get_by_id(id, &authors) {
        let books = get_books_by_author(author.id, &data.books);
        let series = get_series_by_author(author.id, &data.series, &books);
        let context = json!({
            "author": author,
            "series": series,
            "books": books,
            "version": data.version,
            "last_update": data.last_update
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

#[get("/series/<id>")]
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
                "last_update": data.last_update
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

fn get_by_id<T: ID>(id: u32, list: &Vec<T>) -> Option<&T> {
    list.iter().find(|&obj| obj.get_id() == id)
}

fn get_books_by_author(author_id: u32, books: &Vec<Book>) -> Vec<Book> {
    books.iter().filter(|book| book.author_id == author_id).cloned().collect()
}

fn get_series_by_author(author_id: u32, series: &Vec<Series>, books: &Vec<Book>) -> Vec<Series> {
    let mut id_set = HashSet::new();
    books.iter().map(|b| id_set.insert(b.series_id));
    series.iter().filter(|s| id_set.contains(&s.id)).cloned().collect()
}

fn get_books_by_series(series_id: u32, books: &Vec<Book>) -> Vec<Book> {
    books.iter().filter(|book| book.series_id == series_id).cloned().collect()
}
