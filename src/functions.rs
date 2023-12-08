use crate::structs::{Book, Series, ID};
use std::collections::HashSet;

pub fn get_by_id<T: ID>(id: u32, list: &Vec<T>) -> Option<&T> {
    list.iter().find(|&obj| obj.get_id() == id)
}

pub fn get_books_by_author(author_id: u32, books: &Vec<Book>) -> Vec<Book> {
    books
        .iter()
        .filter(|book| book.author_id == author_id)
        .cloned()
        .collect()
}

pub fn get_series_from_books(series: &Vec<Series>, books: &Vec<Book>) -> Vec<Series> {
    let mut id_set = HashSet::new();
    let _ = books.iter().map(|b| id_set.insert(b.series_id));
    series.iter().filter(|s| id_set.contains(&s.id)).cloned().collect()
}

pub fn get_books_by_series(series_id: u32, books: &Vec<Book>) -> Vec<Book> {
    books
        .iter()
        .filter(|book| book.series_id == series_id)
        .cloned()
        .collect()
}
