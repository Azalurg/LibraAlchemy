use clap::Parser;
use regex::Regex;
use rocket::figment::Figment;
use rocket::fs::NamedFile;
use rocket::{get, routes, Rocket, State, Config};
use rocket_dyn_templates::Template;
use routes::*;
use rust_embed::RustEmbed;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tempfile::{tempdir};


mod functions;
mod routes;
mod scanner;
mod structs;

use structs::Library;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

fn load_data_from_json(path: &str) -> Result<Library, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Library = serde_json::from_str(&contents)?;

    Ok(data)
}

fn configure(path: &Path) -> Figment {
    let mut  conf = Config::figment().merge(("port", 4000));
    conf = conf.merge(("template_dir", path));
    conf
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

    let dir = tempdir().unwrap();

    let engine = Template::custom(|engine| {
        let regex = Regex::new(r"\.html\.hbs$").expect("Invalid regex pattern");
        for file in Templates::iter() {
            let name = &regex.replace_all(file.as_ref(), "");
            let file_data = Templates::get(file.as_ref()).unwrap();
            let file_str = std::str::from_utf8(file_data.data.as_ref()).unwrap();
            match engine.handlebars.register_template_string(name, file_str) {
                Ok(_) => println!("Template registered successfully: {}", name),
                Err(e) => eprintln!("Error registering template {}: {}", name, e),
            }
        }

    });

    let _ = rocket::custom(configure(dir.path()))
        .manage(data)
        .manage(dir)
        .mount("/", routes![public::index])
        .mount("/books", routes![books::books, books::book_page])
        .mount("/series", routes![series::series, series::series_page])
        .mount("/authors", routes![authors::authors, authors::author_page])
        .mount("/", routes![static_files])
        .attach(engine)
        .launch()
        .await;
}

// https://github.com/SergioBenitez/Rocket/blob/master/examples/templating/src/hbs.rs
