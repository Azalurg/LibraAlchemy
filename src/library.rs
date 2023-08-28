use crate::structs::Library;
use rocket::State;
use serde_json::to_value;
use self::handlebars::{Handlebars, JsonRender};
use rocket_dyn_templates::{context, handlebars, Template};

#[get("/")]
pub fn index(data: &State<Library>) -> Template {
    let context = to_value(&**data).expect("Failed to serialize data to JSON");

    Template::render(
        "main",
        context
    )
}
