use self::super::{PURSUER_URL, TAIWAN_RECYCLING_SYMBOL_URL};
use gen_epub_book::ops::BookElement;
use std::path::PathBuf;
use chrono::DateTime;
use url::Url;


#[test]
fn name() {
    assert_eq!(&BookElement::Name("ePub test".to_string()).to_string(), "Name: ePub test");
}

#[test]
fn content() {
    assert_eq!(&BookElement::Content(PathBuf::from("test/two/ch1.html")).to_string(),
               "Content: test/two/ch1.html");
}

#[test]
fn string_content() {
    assert_eq!(&BookElement::StringContent("<i>Italics</i>".to_string()).to_string(),
               "String-Content: <i>Italics</i>");
}

#[test]
fn image_content() {
    assert_eq!(&BookElement::ImageContent(PathBuf::from("images/ch1.png")).to_string(),
               "Image-Content: images/ch1.png");
}

#[test]
fn network_image_content() {
    assert_eq!(BookElement::NetworkImageContent(Url::parse(PURSUER_URL).unwrap()).to_string(),
               format!("Network-Image-Content: {}", PURSUER_URL));
}

#[test]
fn cover() {
    assert_eq!(&BookElement::Cover(PathBuf::from("cover.jpg")).to_string(), "Cover: cover.jpg");
}

#[test]
fn network_cover() {
    assert_eq!(BookElement::NetworkImageContent(Url::parse(TAIWAN_RECYCLING_SYMBOL_URL).unwrap()).to_string(),
               format!("Network-Image-Content: {}", TAIWAN_RECYCLING_SYMBOL_URL));
}

#[test]
fn author() {
    assert_eq!(&BookElement::Author("nabijaczleweli".to_string()).to_string(), "Author: nabijaczleweli");
}

#[test]
fn date() {
    assert_eq!(&BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()).to_string(),
               "Date: 2017-02-08T15:30:18+01:00");
}

#[test]
fn language() {
    assert_eq!(&BookElement::Language("en-GB".to_string()).to_string(), "Language: en-GB");
}
