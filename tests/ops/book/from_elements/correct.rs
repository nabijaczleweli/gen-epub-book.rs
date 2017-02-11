use gen_epub_book::ops::{BookElement, EPubContent, EPubCover, EPubBook};
use std::path::PathBuf;
use chrono::DateTime;
use url::Url;


#[test]
fn no_cover() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Ok(EPubBook {
                   name: "Simple ePub demonstration".to_string(),
                   author: "nabijaczleweli".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "en-GB".to_string(),
                   cover: None,
                   content: vec![],
               }));
}

#[test]
fn file_cover() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::Cover(PathBuf::from("examples/cover.png")),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Ok(EPubBook {
                   name: "Simple ePub demonstration".to_string(),
                   author: "nabijaczleweli".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "en-GB".to_string(),
                   cover: Some(EPubCover::File(("cover".to_string(), PathBuf::from("examples/cover.png")))),
                   content: vec![],
               }));
}

#[test]
fn network_cover() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::NetworkCover(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::Language("en-GB".to_string())]),
               Ok(EPubBook {
                   name: "Simple ePub demonstration".to_string(),
                   author: "nabijaczleweli".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "en-GB".to_string(),
                   cover: Some(EPubCover::Network(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap())),
                   content: vec![],
               }));
}

#[test]
fn content_propagation() {
    assert_eq!(EPubBook::from_elements(vec![BookElement::Name("Simple ePub demonstration".to_string()),
                                            BookElement::Content(PathBuf::from("examples/simple/ctnt.html")),
                                            BookElement::Author("nabijaczleweli".to_string()),
                                            BookElement::StringContent("<em>Seize the means of production!</em>".to_string()),
                                            BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                            BookElement::ImageContent(PathBuf::from("examples/simple/chapter_image.png")),
                                            BookElement::Language("en-GB".to_string()),
                                            BookElement::NetworkImageContent(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap())]),
               Ok(EPubBook {
                   name: "Simple ePub demonstration".to_string(),
                   author: "nabijaczleweli".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "en-GB".to_string(),
                   cover: None,
                   content: vec![EPubContent::File(("examples-simple-ctnt".to_string(), PathBuf::from("examples/simple/ctnt.html"))),
                                 EPubContent::String("<em>Seize the means of production!</em>".to_string()),
                                 EPubContent::Image(("examples-simple-chapter_image".to_string(), PathBuf::from("examples/simple/chapter_image.png"))),
                                 EPubContent::NetworkImage(Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap())],
               }));
}
