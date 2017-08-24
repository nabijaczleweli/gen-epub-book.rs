use gen_epub_book::ops::IncludeDirectory;
use std::path::PathBuf;


#[test]
fn unnamed() {
    assert_eq!(IncludeDirectory::Unnamed { dir: ("asdf/fdsa\\zxcv".to_string(), PathBuf::from("")) }.to_string(),
               "asdf/fdsa\\zxcv");
}

#[test]
fn named() {
    assert_eq!(IncludeDirectory::Named {
                       name: "well".to_string(),
                       dir: ("asdf/fdsa\\zxcv".to_string(), PathBuf::from("")),
                   }
                   .to_string(),
               "well=asdf/fdsa\\zxcv");
}
