use self::super::super::{PURSUER_URL, TAIWAN_RECYCLING_SYMBOL_URL};
use gen_epub_book::ops::BookElement;
use std::path::PathBuf;
use chrono::DateTime;
use url::Url;


#[test]
fn name() {
    assert_eq!(BookElement::parse("Name: ePub test", ":", false),
               Ok(Some(BookElement::Name("ePub test".to_string()))));
}

#[test]
fn content() {
    assert_eq!(BookElement::parse("Content: test/two/ch1.html", ":", false),
               Ok(Some(BookElement::Content(PathBuf::from("test/two/ch1.html")))));
}

#[test]
fn string_content() {
    assert_eq!(BookElement::parse("String-Content: <i>Italics</i>", ":", false),
               Ok(Some(BookElement::StringContent("<i>Italics</i>".to_string()))));
}

#[test]
fn image_content() {
    assert_eq!(BookElement::parse("Image-Content: images/ch1.png", ":", false),
               Ok(Some(BookElement::ImageContent(PathBuf::from("images/ch1.png")))));
}

#[test]
fn network_image_content() {
    assert_eq!(BookElement::parse(&format!("Network-Image-Content: {}", PURSUER_URL), ":", false),
               Ok(Some(BookElement::NetworkImageContent(Url::parse(PURSUER_URL).unwrap()))));
}

#[test]
fn cover() {
    assert_eq!(BookElement::parse("Cover: cover.jpg", ":", false),
               Ok(Some(BookElement::Cover(PathBuf::from("cover.jpg")))));
}

#[test]
fn network_cover() {
    assert_eq!(BookElement::parse(&format!("Network-Cover: {}", TAIWAN_RECYCLING_SYMBOL_URL), ":", false),
               Ok(Some(BookElement::NetworkCover(Url::parse(TAIWAN_RECYCLING_SYMBOL_URL).unwrap()))));
}

#[test]
fn include() {
    assert_eq!(BookElement::parse("Include: style.css", ":", false),
               Ok(Some(BookElement::Include(PathBuf::from("style.css")))));
}

#[test]
fn network_include() {
    assert_eq!(BookElement::parse(&format!("Network-Include: {}", TAIWAN_RECYCLING_SYMBOL_URL), ":", false),
               Ok(Some(BookElement::NetworkInclude(Url::parse(TAIWAN_RECYCLING_SYMBOL_URL).unwrap()))));
}

#[test]
fn author() {
    assert_eq!(BookElement::parse("Author: nabijaczleweli", ":", false),
               Ok(Some(BookElement::Author("nabijaczleweli".to_string()))));
}

#[test]
fn date_rigid() {
    assert_eq!(BookElement::parse("Date: 2017-02-08T15:30:18+01:00", ":", false),
               Ok(Some(BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()))));
}

#[test]
fn date_free() {
    assert_eq!(BookElement::parse("Date: 2017-02-08T15:30:18+01:00", ":", true),
               Ok(Some(BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()))));
    assert_eq!(BookElement::parse("Date: Wed, 08 Feb 2017 15:30:18 +0100", ":", true),
               Ok(Some(BookElement::Date(DateTime::parse_from_rfc2822("Wed, 08 Feb 2017 15:30:18 +0100").unwrap()))));
    assert_eq!(BookElement::parse("Date: 1486564218+0100", ":", true),
               Ok(Some(BookElement::Date(DateTime::parse_from_str("1486564218+0100", "%s%:z").unwrap()))));
}

#[test]
fn language() {
    assert_eq!(BookElement::parse("Language: en-GB", ":", false),
               Ok(Some(BookElement::Language("en-GB".to_string()))));
}
