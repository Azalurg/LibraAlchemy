use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub directory: String,
    pub cover: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub title: String,
    pub books: Vec<Book>,
    pub cover: String,
    pub directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub albums: Vec<Album>,
    pub books: Vec<Book>,
    pub cover: String,
    pub directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub authors: Vec<Author>,
    pub books_amount: u32,
    pub version: String
}