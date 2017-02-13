use gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
use std::path::PathBuf;
use chrono::DateTime;
use url::Url;


#[test]
fn no_cover() {
    let book = EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())])
        .unwrap();

    assert_eq!(book.name, "Simple ePub demonstration".to_string());
    assert_eq!(book.author, "nabijaczleweli".to_string());
    assert_eq!(book.date, DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap());
    assert_eq!(book.language, "en-GB".to_string());
    assert_eq!(book.cover, None);
}

#[test]
fn file_cover() {
    let book = EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Cover(PathBuf::from("examples/cover.png")),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())])
        .unwrap();

    assert_eq!(book.name, "Simple ePub demonstration".to_string());
    assert_eq!(book.author, "nabijaczleweli".to_string());
    assert_eq!(book.date, DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap());
    assert_eq!(book.language, "en-GB".to_string());
    assert_eq!(book.cover,
               Some(("examples-cover".to_string(), PathBuf::from("examples-cover.png"), EPubContentType::File(PathBuf::from("examples/cover.png")))));
}

#[test]
fn network_cover() {
    let book = EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::NetworkCover(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())])
        .unwrap();

    assert_eq!(book.name, "Simple ePub demonstration".to_string());
    assert_eq!(book.author, "nabijaczleweli".to_string());
    assert_eq!(book.date, DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap());
    assert_eq!(book.language, "en-GB".to_string());
    assert_eq!(book.cover,
               Some(("network-cover-ViQ2WED".to_string(),
                     PathBuf::from("ViQ2WED.jpg"),
                     EPubContentType::Network(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()))));
}
