use gen_epub_book::ops::{BookElement, EPubBook};
use gen_epub_book::Error;
use std::path::PathBuf;
use chrono::DateTime;
use url::Url;


#[test]
fn two_file() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Cover(PathBuf::from("examples/cover.png")),
                                            BookElement::Cover(PathBuf::from("examples/cover.png")),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Cover and Network-Cover",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}

#[test]
fn two_network() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::NetworkCover(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()),
                                            BookElement::NetworkCover(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Cover and Network-Cover",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}

#[test]
fn mixed_file_first() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Cover(PathBuf::from("examples/cover.png")),
                                            BookElement::NetworkCover(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Cover and Network-Cover",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}

#[test]
fn mixed_network_first() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::NetworkCover(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()),
                                            BookElement::Cover(PathBuf::from("examples/cover.png")),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Cover and Network-Cover",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}
