use crate::structs::Library;
use rocket::{get, State};
use rocket_dyn_templates::Template;
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
