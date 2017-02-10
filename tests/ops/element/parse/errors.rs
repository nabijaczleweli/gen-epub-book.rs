use gen_epub_book::ops::BookElement;
use gen_epub_book::Error;


#[test]
fn url() {
    assert_eq!(BookElement::parse("Network-Cover: http/i.imgur.com/ViQ2WED.jpg"),
               Err(Error::Parse {
                   tp: "URL",
                   wher: "book element",
                   more: None,
               }));
}

#[test]
fn datetime() {
    assert_eq!(BookElement::parse("Date: Thu, 29 Dec 2016 15:37:19 +0100"),
               Err(Error::Parse {
                   tp: "datetime",
                   wher: "book element",
                   more: Some("not RFC3339"),
               }));
}
