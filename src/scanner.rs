use serde_json;
use std::fs;
use walkdir::WalkDir;
use rand::Rng;

use crate::structs::{Author, Book, Library, Series};

fn get_cover_path(path: String) -> String {
    let exts = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".nfo"];
    let names = ["cover", "folder", "album", "poster", "default", "art"];

    for ext in exts.iter() {
        for name in names.iter() {
            let cover_path = path.clone() + "/" + name + ext;
            if fs::metadata(&cover_path).is_ok() {
                return cover_path;
            }
        }
    }
    String::new()
}

fn contains_subfolders(folder_path: &str) -> bool {
    match fs::read_dir(folder_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        return true;
                    }
                }
            }
            false
        },
        Err(_) => false, // Return false on error (directory not found, permissions, etc.)
    }
}

fn get_name(dir: &walkdir::DirEntry) -> String {
    dir.path().file_name().unwrap().to_str().unwrap().to_string()
}


pub fn scan(work_dir: &str, json_path: &str) {
    println!("Scanning: {}", work_dir);

    let mut lib = Library::new();

    for author in WalkDir::new(work_dir)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
    {
        let author_name = get_name(&author);
        let mut author_books = 0;
        let mut series_amount = 0;
        let author_id = lib.authors_amount + 1 ;

        for series in WalkDir::new(author.path().clone())
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
        {
            if contains_subfolders(series.path().display().to_string().as_str()) {
                let mut series_books = 0;
                let series_name = get_name(&series);
                let series_id: u32 = lib.series_amount + 1;

                for book in WalkDir::new(series.path().clone())
                    .max_depth(1)
                    .min_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                {
                    lib.add_book(Book {
                        title: get_name(&book),
                        directory: book.path().display().to_string(),
                        cover: get_cover_path(book.path().display().to_string()),
                        author: author_name.clone(),
                        series: series_name.clone(),
                        id: lib.books_amount + 1,
                        series_id: series_id.clone(),
                        author_id: author_id.clone(),
                    });
                    series_books += 1;
                    author_books += 1;
                }

                lib.add_series(Series {
                    title: series_name,
                    cover: get_cover_path(series.path().display().to_string()),
                    directory: series.path().display().to_string(),
                    author: author_name.clone(),
                    books_amount: series_books,
                    id: series_id,
                });
                series_amount += 1;
            } else {
                let book = series;
                lib.add_book(Book {
                    title: get_name(&book),
                    directory: book.path().display().to_string(),
                    cover: get_cover_path(book.path().display().to_string()),
                    author: author_name.clone(),
                    series: String::new(),
                    id: lib.books_amount + 1,
                    series_id: 0,
                    author_id: author_id.clone(),
                });
                author_books += 1;
            }
        }

        lib.add_author(Author {
            name: author_name,
            cover: get_cover_path(author.path().display().to_string()),
            directory: author.path().display().to_string(),
            series_amount: series_amount,
            books_amount: author_books,
            id: author_id,
        });
    }

    let json_data = serde_json::to_string_pretty(&lib).expect("Failed to serialize data to JSON");
    std::fs::write(json_path, json_data).expect("Failed to write JSON data to file");
    print!("JSON data saved to: {}", json_path);
    println!("Scanning complied");
}
