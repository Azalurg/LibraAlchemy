use rocket::fs::NamedFile;
use rocket::{routes, Rocket, get, State};
use rocket_dyn_templates::Template;
use serde_json;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};

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

#[get("/<file..>", rank = 10)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("/").join(file)).await.ok()
}

#[rocket::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut work_dir = String::from(env::current_dir().unwrap().display().to_string());
    let mut data_store = String::from("/etc/LibraAlchemy");

    match args.len() {
        1 => {
            work_dir = String::from(args[0].clone());
        },
        2 => {
            work_dir = String::from(args[0].clone());
            data_store = String::from(args[1].clone());
        },
        _ => {},
    }

    if work_dir.chars().last().unwrap() != '/' {
        work_dir = work_dir + "/";
    }

    if data_store.chars().last().unwrap() != '/' {
        data_store = data_store + "/";
    }

    let json_path = data_store.clone() + "data.json";

    println!("--- START ---");

    scanner::scan(&work_dir.clone(), &json_path);  //  <--- For development
    let data = load_data_from_json(&json_path).unwrap();

    println!("--- END ---");

    let _ = rocket::build()
        .manage(data)
        .mount("/", routes![templates::index, templates::statics, templates::books])
        .mount("/", routes![static_files])
        .attach(Template::fairing())
        .launch()
        .await;
}

// https://github.com/SergioBenitez/Rocket/blob/master/examples/templating/src/hbs.rs
