use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub directory: String,
    pub cover: String,
    pub author: String,
    pub album: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub title: String,
    pub cover: String,
    pub directory: String,
    pub author: String,
    pub books_amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub cover: String,
    pub directory: String,
    pub albums_amount: u32,
    pub books_amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub authors: Vec<Author>,
    pub albums: Vec<Album>,
    pub books: Vec<Book>,
    pub authors_amount: u32,
    pub albums_amount: u32,
    pub books_amount: u32,
    pub version: String,
}

impl Library {
    pub fn new() -> Library {
        Library {
            authors: Vec::new(),
            albums: Vec::new(),
            books: Vec::new(),
            authors_amount: 0,
            albums_amount: 0,
            books_amount: 0,
            version: String::from(format!("{}", crate::VERSION)),
        }
    }

    pub fn add_author(&mut self, author: Author) {
        self.authors.push(author);
        self.authors_amount += 1;
    }

    pub fn add_album(&mut self, album: Album) {
        self.albums.push(album);
        self.albums_amount += 1;
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
        self.books_amount += 1;
    }
}
