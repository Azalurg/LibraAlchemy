use std::fs;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    directory: String,
    cover: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Album {
    title: String,
    books: Vec<Book>,
    cover: String,
    directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String,
    albums: Vec<Album>,
    books: Vec<Book>,
    cover: String,
    directory: String,
}

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

fn main() {
    let work_dir = "/mnt/d/Audiobooks/Test"; // Change this to your desired working directory

    let mut book_lib: Vec<Author> = Vec::new();

    for entry in WalkDir::new(work_dir)
        .max_depth(1) // +1 to include the desired level
        .min_depth(1) // Ensure we're only at the desired level
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
            .max_depth(1) // +1 to include the desired level
            .min_depth(1) // Ensure we're only at the desired level
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
                    .max_depth(1) // +1 to include the desired level
                    .min_depth(1) // Ensure we're only at the desired level
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                {
                    book_lib.last_mut().unwrap().albums.last_mut().unwrap().books.push(Book {
                        title: album.path().file_name().unwrap().to_str().unwrap().to_string(),
                        directory: album.path().display().to_string(),
                        cover: check_cover(album.path().display().to_string() + "/cover.jpg"),
                    });
                } // +1 to include the desired level
            } else {
                book_lib.last_mut().unwrap().books.push(Book {
                    title: author.path().file_name().unwrap().to_str().unwrap().to_string(),
                    directory: author.path().display().to_string(),
                    cover: check_cover(author.path().display().to_string() + "/cover.jpg"),
                });
            }
        }
    }

    let json_data = serde_json::to_string_pretty(&book_lib).expect("Failed to serialize data to JSON");
    std::fs::write("./assets/lib.json", json_data).expect("Failed to write JSON data to file");
}


// for author in book_lib.iter() {
    //     println!("Author: {}", author.name);
    //     for album in author.albums.iter() {
    //         println!("  - Album: {}", album.title);
    //         for book in album.books.iter() {
    //             println!("      - Book: {}", book.title);
    //         }
    //     }
    //     for book in author.books.iter() {
    //         println!("  - Book: {}", book.title);
    //     }
    // }