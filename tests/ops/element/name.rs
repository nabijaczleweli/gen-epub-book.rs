use self::super::{PURSUER_URL, TAIWAN_RECYCLING_SYMBOL_URL};
use gen_epub_book::ops::BookElement;
use std::path::PathBuf;
use chrono::DateTime;
use url::Url;


#[test]
fn name() {
    assert_eq!(BookElement::Name("ePub test".to_string()).name(), "Name");
}

#[test]
fn content() {
    assert_eq!(BookElement::Content(PathBuf::from("test/two/ch1.html")).name(), "Content");
}

#[test]
fn string_content() {
    assert_eq!(BookElement::StringContent("<i>Italics</i>".to_string()).name(), "String-Content");
}

#[test]
fn image_content() {
    assert_eq!(BookElement::ImageContent(PathBuf::from("images/ch1.png")).name(), "Image-Content");
}

#[test]
fn network_image_content() {
    assert_eq!(BookElement::NetworkImageContent(Url::parse(PURSUER_URL).unwrap()).name(),
               "Network-Image-Content");
}

#[test]
fn cover() {
    assert_eq!(BookElement::Cover(PathBuf::from("cover.jpg")).name(), "Cover");
}

#[test]
fn network_cover() {
    assert_eq!(BookElement::NetworkImageContent(Url::parse(TAIWAN_RECYCLING_SYMBOL_URL).unwrap()).name(),
               "Network-Image-Content");
}

#[test]
fn include() {
    assert_eq!(BookElement::Include(PathBuf::from("style.css")).name(), "Include");
}

#[test]
fn network_include() {
    assert_eq!(BookElement::NetworkInclude(Url::parse(TAIWAN_RECYCLING_SYMBOL_URL).unwrap()).name(),
               "Network-Include");
}

#[test]
fn author() {
    assert_eq!(BookElement::Author("nabijaczleweli".to_string()).name(), "Author");
}

#[test]
fn date() {
    assert_eq!(BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()).name(),
               "Date");
}

#[test]
fn language() {
    assert_eq!(BookElement::Language("en-GB".to_string()).name(), "Language");
}
