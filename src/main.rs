use clap::Parser;
use rocket::figment::Figment;
use rocket::fs::NamedFile;
use rocket::{get, routes, Config};
use rocket_dyn_templates::Template;
use routes::*;
use rust_embed::RustEmbed;
use serde_json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::tempdir;

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
    let mut conf = Config::figment().merge(("port", 8000));
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

    let tmp_dir = tempdir().unwrap();
    let tmp_dir_path = tmp_dir.path();

    for file in Templates::iter() {
        let path = tmp_dir_path.join(file.as_ref());
        let _ = fs::create_dir_all(std::path::Path::new(&path).parent().unwrap());

        let mut tmp_file = File::create(tmp_dir_path.join(file.as_ref())).unwrap();
        let file_content = Templates::get(file.as_ref()).unwrap();
        tmp_file.write_all(file_content.data.as_ref()).unwrap();
    }

    let _ = rocket::custom(configure(tmp_dir_path))
        .manage(data)
        .manage(tmp_dir)
        .mount("/", routes![public::index])
        .mount("/books", routes![books::books, books::book_page])
        .mount("/series", routes![series::series, series::series_page])
        .mount("/authors", routes![authors::authors, authors::author_page])
        .mount("/", routes![static_files])
        .attach(Template::fairing())
        .launch()
        .await;
}

// https://github.com/SergioBenitez/Rocket/blob/master/examples/templating/src/hbs.rs
