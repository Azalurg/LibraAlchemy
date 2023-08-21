use std::fs;
use walkdir::WalkDir;
use serde_json;

use crate::structs::{Author, Album, Book, Library};

fn check_cover(path: String) -> String {
    if fs::metadata(&path).is_ok() {
        path
    } else {
        String::new()
    }
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
        }
        Err(_) => false, // Return false on error (directory not found, permissions, etc.)
    }
}

pub fn scan(work_dir: &str, json_path: &str){
    println!("Scanning: {}", work_dir);

    let mut book_lib: Vec<Author> = Vec::new();
    let mut books_amount: u32 = 0;

    for entry in WalkDir::new(work_dir)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
    {
        let author_name = entry.path().file_name().unwrap().to_str().unwrap().to_string();

        book_lib.push(Author {
            name: author_name,
            albums: Vec::new(),
            books: Vec::new(),
            cover: check_cover(entry.path().display().to_string() + "/cover.jpg"),
            directory: entry.path().display().to_string(),
        });

        for author in WalkDir::new(book_lib.last().unwrap().directory.clone())
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
        {
            if contains_subfolders(author.path().display().to_string().as_str()) {
                book_lib.last_mut().unwrap().albums.push(Album {
                    title: author.path().file_name().unwrap().to_str().unwrap().to_string(),
                    books: Vec::new(),
                    cover: check_cover(author.path().display().to_string() + "/cover.jpg"),
                    directory: author.path().display().to_string(),
                });

                for album in WalkDir::new(book_lib.last().unwrap().albums.last().unwrap().directory.clone())
                    .max_depth(1)
                    .min_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                {
                    book_lib.last_mut().unwrap().albums.last_mut().unwrap().books.push(Book {
                        title: album.path().file_name().unwrap().to_str().unwrap().to_string(),
                        directory: album.path().display().to_string(),
                        cover: check_cover(album.path().display().to_string() + "/cover.jpg"),
                    });
                    books_amount += 1;
                }
            } else {
                book_lib.last_mut().unwrap().books.push(Book {
                    title: author.path().file_name().unwrap().to_str().unwrap().to_string(),
                    directory: author.path().display().to_string(),
                    cover: check_cover(author.path().display().to_string() + "/cover.jpg"),
                });
                books_amount += 1;
            }
        }
    }
    let lib = Library {
        authors: book_lib,
        books_amount: books_amount,
        version: String::from(format!("{}", crate::VERSION)),
    };

    let json_data = serde_json::to_string_pretty(&lib).expect("Failed to serialize data to JSON");
    std::fs::write(json_path, json_data).expect("Failed to write JSON data to file");
    println!("Scanning complied");
}