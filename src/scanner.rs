use serde_json;
use std::fs;
use walkdir::WalkDir;

use crate::structs::{Album, Author, Book, Library};

fn get_cover_path(path: String) -> String {
    let ext = [".jpg", ".jpeg", ".png", ".gif", ".webp"];
    for i in ext.iter() {
        let cover_path = path.clone() + "/cover" + i;
        if fs::metadata(&cover_path).is_ok() {
            return cover_path;
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
        let mut albums_amount = 0;

        for album in WalkDir::new(author.path().clone())
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
        {
            if contains_subfolders(album.path().display().to_string().as_str()) {
                let mut album_books = 0;
                let album_name = get_name(&album);

                for book in WalkDir::new(album.path().clone())
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
                        album: album_name.clone(),
                    });
                    album_books += 1;
                    author_books += 1;
                }

                lib.add_album(Album {
                    title: album_name,
                    cover: get_cover_path(album.path().display().to_string()),
                    directory: album.path().display().to_string(),
                    author: author_name.clone(),
                    books_amount: album_books,
                });
                albums_amount += 1;
            } else {
                let book = album;
                lib.add_book(Book {
                    title: get_name(&book),
                    directory: book.path().display().to_string(),
                    cover: get_cover_path(book.path().display().to_string()),
                    author: author_name.clone(),
                    album: String::new(),
                });
                author_books += 1;
            }
        }

        lib.add_author(Author {
            name: author_name,
            cover: get_cover_path(author.path().display().to_string()),
            directory: author.path().display().to_string(),
            albums_amount: albums_amount,
            books_amount: author_books,
        });
    }

    let json_data = serde_json::to_string_pretty(&lib).expect("Failed to serialize data to JSON");
    std::fs::write(json_path, json_data).expect("Failed to write JSON data to file");
    print!("JSON data saved to: {}", json_path);
    println!("Scanning complied");
}
