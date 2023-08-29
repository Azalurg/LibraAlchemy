use std::env;
use rocket_dyn_templates::Template;
use std::fs::File;
use std::io::Read;
use serde_json;

mod scanner;
mod structs;
mod templates;

use structs::Library;

pub const VERSION: &str = "0.3.3";

fn load_data_from_json(path: &str) -> Result<Library, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Library = serde_json::from_str(&contents)?;

    Ok(data)
}

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut work_dir = String::new();

    if args.len() >= 1 {
        work_dir = args[0].clone();
        println!("Working directory: {}", args[0]);
    } else {
        work_dir = String::from("./");
        println!("No working directory specified");
    }

    if work_dir.chars().last().unwrap() != '/' {
        work_dir = work_dir + "/";
    }

    let json_path = work_dir.clone() + "data.json";

    println!("--- START ---");

    scanner::scan(&work_dir.clone(), &json_path);
    let data = load_data_from_json(&json_path).unwrap();

    println!("--- END ---");

    rocket::build()
        .manage(data)
        .mount("/", routes![templates::index])
        .attach(Template::fairing())
}

// https://github.com/SergioBenitez/Rocket/blob/master/examples/templating/src/hbs.rs
