use clap::Parser;
use rocket::fs::NamedFile;
use rocket::{get, routes, Rocket, State};
use rocket_dyn_templates::Template;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;

mod scanner;
mod structs;
mod templates;

use structs::Library;

pub const VERSION: &str = "1.0.0";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./")]
    work_dir: String,
    #[arg(short, long, default_value = "/tmp/")]
    output_dir: String,
    #[arg(short, default_value = "false")]
    gen_static: bool,
}

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
    let args = Args::parse();

    let mut work_dir = args.work_dir;
    let mut data_store = args.output_dir;

    if work_dir.chars().last().unwrap() != '/' {
        work_dir = work_dir + "/";
    }

    if data_store.chars().last().unwrap() != '/' {
        data_store = data_store + "/";
    }

    let json_path = data_store.clone() + "LibraAlchemy.json";

    println!("--- START ---");

    scanner::scan(&work_dir.clone(), &json_path); //  <--- For development
    let data = load_data_from_json(&json_path).unwrap();

    if args.gen_static {
        panic!("Template generated successfully!");
    }

    println!("--- END ---");

    let _ = rocket::build()
        .manage(data)
        .mount(
            "/",
            routes![
                templates::index,
                templates::statics,
                templates::books,
                templates::authors,
                templates::series,
                templates::book_page,
                templates::author_page,
                templates::series_page
            ],
        )
        .mount("/", routes![static_files])
        .attach(Template::fairing())
        .launch()
        .await;
}

// https://github.com/SergioBenitez/Rocket/blob/master/examples/templating/src/hbs.rs
