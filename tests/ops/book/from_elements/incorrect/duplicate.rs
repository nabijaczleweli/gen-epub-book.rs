use gen_epub_book::ops::{BookElement, EPubBook};
use gen_epub_book::Error;
use chrono::DateTime;


#[test]
fn name() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Name",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}


#[test]
fn author() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Author",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}

#[test]
fn date() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Date",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}

#[test]
fn language() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Language",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}

#[test]
fn ordering() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Language("en-GB".to_string()),
                                            BookElement::Language("en-GB".to_string())]),
               Err(Error::WrongElementAmount {
                   element: "Author",
                   actual: 2,
                   relation: "exactly",
                   bound: 1,
               }));
}
