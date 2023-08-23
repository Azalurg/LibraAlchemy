mod raw;

pub struct Templates {
    pub main: String,
    pub author: String,
    pub series: String,
    pub book: String,
}

#[cfg(feature = "external-templates")]
pub fn get_templates() -> Templates {
    use self::raw::{TEMP_AUTHOR, TEMP_BOOK, TEMP_MAIN, TEMP_SERIES};

    Templates {
        main: TEMP_MAIN.to_string(),
        author: TEMP_AUTHOR.to_string(),
        series: TEMP_SERIES.to_string(),
        book: TEMP_BOOK.to_string(),
    }
}

#[cfg(not(feature = "external-templates"))]
pub fn get_templates() -> Templates {
    use std::fs;

    let main = fs::read_to_string("assets/templates/main.hbs").expect("Failed to read template file");
    let author = fs::read_to_string("assets/templates/author.hbs").expect("Failed to read template file");
    let series = fs::read_to_string("assets/templates/series.hbs").expect("Failed to read template file");
    let book = fs::read_to_string("assets/templates/book.hbs").expect("Failed to read template file");

    Templates {
        main,
        author,
        series,
        book,
    }
}
