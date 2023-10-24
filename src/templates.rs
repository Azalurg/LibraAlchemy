use self::handlebars::{Handlebars, JsonRender};
use crate::structs::{Library, ID};
use rocket::{get, State};
use rocket_dyn_templates::{context, handlebars, Template};
use serde_json::to_value;

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
pub fn book(id: u32, data: &State<Library>) -> Template {
    let books = &data.books;

    if let Some(book) = get_by_id(id, &books) {
        Template::render("pages/books/details", book)
    } else {
        Template::render("pages/error", id)
    }
}

fn get_by_id<T: ID>(id: u32, list: &Vec<T>) -> Option<&T>{
    list.iter().find(|&obj| obj.get_id() == id)
}