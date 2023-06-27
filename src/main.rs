#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fs};

lazy_static! {
    static ref BOOK_RECORDS: HashMap<String, BookRecord> = {
        let mut map = HashMap::new();
        let records = parse_book_records("goodreads.json");
        for record in records {
            map.insert(record.title.clone(), record);
        }
        map
    };
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct BookRecordReponse(String);

#[derive(Serialize, Deserialize, Debug)]
struct BookRecord {
    #[serde(alias = "Title")]
    title: String,
    #[serde(alias = "Author")]
    author: String,
    #[serde(alias = "Book Id")]
    book_id: u32,
    #[serde(alias = "Original Publication Year")]
    year_published: u32,
}

fn parse_book_records(path: &str) -> Vec<BookRecord> {
    let contents = fs::read_to_string(path).expect("Unabled to open book record file");
    let records = serde_json::from_str(&contents).expect("Unabled to parse json");
    records
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/book/title/<title>")]
fn book_by_title(title: &str) -> Option<BookRecordReponse> {
    BOOK_RECORDS
        .get(title)
        .and_then(|r| Some(BookRecordReponse(serde_json::to_string(r).unwrap())))
}

#[get("/book/year/<year>")]
fn book_by_year(year: u32) -> Option<BookRecordReponse> {
    let results: Vec<&BookRecord> = BOOK_RECORDS
        .values()
        .filter(|b| b.year_published == year)
        .collect();

    Some(BookRecordReponse(serde_json::to_string(&results).unwrap()))
}

#[launch]
fn rocket() -> _ {
    let records = parse_book_records("goodreads.json");
    print!("{:?}", records);
    rocket::build().mount("/", routes![index, book_by_title, book_by_year])
}