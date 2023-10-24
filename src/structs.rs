use chrono::Local;
use serde::{Deserialize, Serialize};

pub trait ID {
    fn get_id(&self) -> u32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub series_id: u32,
    pub author_id: u32,
    pub title: String,
    pub directory: String,
    pub cover: String,
    pub author: String,
    pub series: String,
}

impl ID for Book {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub id: u32,
    pub title: String,
    pub cover: String,
    pub directory: String,
    pub author: String,
    pub books_amount: u32,
}

impl ID for Series {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: u32,
    pub name: String,
    pub cover: String,
    pub directory: String,
    pub series_amount: u32,
    pub books_amount: u32,
}

impl ID for Author {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub authors: Vec<Author>,
    pub series: Vec<Series>,
    pub books: Vec<Book>,
    pub authors_amount: u32,
    pub series_amount: u32,
    pub books_amount: u32,
    pub version: String,
    pub last_update: String,
}

impl Library {
    pub fn new() -> Library {
        let last_update = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Library {
            authors: Vec::new(),
            series: Vec::new(),
            books: Vec::new(),
            authors_amount: 0,
            series_amount: 0,
            books_amount: 0,
            version: String::from(format!("{}", crate::VERSION)),
            last_update: last_update,
        }
    }

    pub fn add_author(&mut self, author: Author) {
        self.authors.push(author);
        self.authors_amount += 1;
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
        self.series_amount += 1;
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
        self.books_amount += 1;
    }
}
