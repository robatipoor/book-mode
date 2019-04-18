//! Simple `.books` parser module

use regex::Regex;

/// A simple wrapper around writing years and year-ranges
#[derive(Debug, PartialEq, Default)]
pub struct Time {
    start: u16,
    end: u16,
}

#[derive(Debug, PartialEq, Default)]
pub struct Book {
    name: String,
    time: Time,
    isbn: Option<String>,
    pages: Option<usize>,
    genre: Option<String>,
    author: Option<String>,
}

struct Books {
    inner: Vec<Book>,
}

impl Books {
    /// Parse a `.books` file into a collection of book metadata
    pub fn parse<'input, I: Into<&'input str>>(input: I) -> Self {
        let lines: Vec<&str> = input.into().split('\n').collect();
        let mut books = vec![];

        for l in lines {
            match l.trim() {
                s if s.to_lowercase().starts_with("isbn:") => {
                    let book: &mut Book = books.last_mut().unwrap();
                    let chunks: Vec<_> = s.split(":").collect();
                    book.isbn = Some(chunks.get(1).unwrap().trim().into());
                }
                s if s.to_lowercase().starts_with("genre:") => {
                    let book: &mut Book = books.last_mut().unwrap();
                    let chunks: Vec<_> = s.split(":").collect();
                    book.genre = Some(chunks.get(1).unwrap().trim().into());
                }
                s if s.to_lowercase().starts_with("pages:") => {
                    let book: &mut Book = books.last_mut().unwrap();
                    let chunks: Vec<_> = s.split(":").collect();
                    book.pages = Some(chunks.get(1).unwrap().trim().parse().unwrap());
                }
                s if s.to_lowercase().starts_with("author:") => {
                    let book: &mut Book = books.last_mut().unwrap();
                    let chunks: Vec<_> = s.split(":").collect();
                    book.author = Some(chunks.get(1).unwrap().trim().into());
                }
                // If it's a `:` key but we don't know it
                key if key.contains(":") => eprintln!("Unknown key `{}`", key),

                // If it's _none_ of these, we assume it's a new book title
                title => {
                    if title.trim() == "" {
                        continue;
                    }

                    let re = Regex::new(r"\t|    ").unwrap();
                    let chunks: Vec<_> = re.split(title).collect();
                    let name = chunks.get(0).expect("No name").to_owned().into();

                    // Chop out the year
                    let ys: String = chunks
                        .iter()
                        .skip(1)
                        .filter(|s| *s != &"")
                        .take(1)
                        .map(|s| s.to_owned())
                        .collect();

                    let time = if ys.contains("-") {
                        let v: Vec<_> = ys.split("-").map(|s| s.trim()).collect();
                        let y1 = v.get(0).unwrap().parse().expect("Failed to parse year");
                        let y2 = v.get(1).unwrap().parse().expect("Failed to parse year");

                        Time { start: y1, end: y2 }
                    } else {
                        let year = ys.trim().parse().expect("Failed to parse year");
                        Time {
                            start: year,
                            end: year,
                        }
                    };

                    books.push(Book {
                        name,
                        time,
                        ..Default::default()
                    });
                }
            }
        }

        Books { inner: books }
    }
}

#[test]
pub fn manual_test() {
    let s = "
The Player of Games				2017
    Genre: Science Fiction
    Pages: 466
    Author: Iain Banks

Accelerando					2017-2018
    Genre: Science Fiction
    Pages: 621
    Author: Charles Stross
";

    let result = Books::parse(s);

    let b1 = Book {
        name: "The Player of Games".into(),
        time: Time {
            start: 2017,
            end: 2017,
        },
        isbn: None,
        pages: Some(466),
        genre: Some("Science Fiction".into()),
        author: Some("Iain Banks".into()),
    };
    let b2 = Book {
        name: "Accelerando".into(),
        time: Time {
            start: 2017,
            end: 2018,
        },
        isbn: None,
        pages: Some(621),
        genre: Some("Science Fiction".into()),
        author: Some("Charles Stross".into()),
    };

    let expect = vec![b1, b2];

    assert_eq!(result.inner, expect);
}
