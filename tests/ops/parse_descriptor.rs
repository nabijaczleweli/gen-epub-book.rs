use gen_epub_book::ops::{BookElement, parse_descriptor};
use gen_epub_book::Error;
use std::path::PathBuf;
use chrono::DateTime;


#[test]
fn bubbling() {
    assert_eq!(parse_descriptor("test string", &mut &b"Date: Thu, 29 Dec 2016 15:37:19 +0100\n"[..], ":"),
               Err(Error::Parse {
                   tp: "datetime",
                   wher: "book element",
                   more: Some("not RFC3339"),
               }));
    assert_eq!(parse_descriptor("test string", &mut &b"Network-Cover: http/i.imgur.com/ViQ2WED.jpg\n"[..], ":"),
               Err(Error::Parse {
                   tp: "URL",
                   wher: "book element",
                   more: None,
               }));
}

#[test]
fn generic() {
    assert_eq!(parse_descriptor("test string",
                                &mut &b"
comment

#comment 2
what: ever

Name: Simple ePub demonstration
Cover: cover.png

Image-Content: examples/simple/chapter_image.png
Content: simple/ctnt.html

Author: nabijaczleweli
Date: 2017-02-08T15:30:18+01:00
Language: en-GB
"
                                          [..],
                                ":"),
               Ok(parsed()));

    assert_eq!(parse_descriptor("test string",
                                &mut &b"
comment

#comment 2
what - - - > ever

Name - - - > Simple ePub demonstration
Cover - - - > cover.png

Image-Content - - - > examples/simple/chapter_image.png
Content - - - > simple/ctnt.html

Author - - - > nabijaczleweli
Date - - - > 2017-02-08T15:30:18+01:00
Language - - - > en-GB
"
                                          [..],
                                "- - - >"),
               Ok(parsed()));
}


fn parsed() -> Vec<BookElement> {
    vec![BookElement::Name("Simple ePub demonstration".to_string()),
         BookElement::Cover(PathBuf::from("cover.png")),
         BookElement::ImageContent(PathBuf::from("examples/simple/chapter_image.png")),
         BookElement::Content(PathBuf::from("simple/ctnt.html")),
         BookElement::Author("nabijaczleweli".to_string()),
         BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
         BookElement::Language("en-GB".to_string())]
}
