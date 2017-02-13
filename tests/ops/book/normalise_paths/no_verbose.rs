use gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
use self::super::make_test_folder;
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use std::env::temp_dir;
use chrono::DateTime;


#[test]
fn correct() {
    let tf = make_test_folder("no-verbose-correct");
    let _ = fs::create_dir(tf.join("content"));
    File::create(tf.join("cover.png")).unwrap();
    File::create(tf.join("content").join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-no-verbose-correct/".to_string(), tf.clone()), false, &mut buf),
               Ok(()));
    assert_eq!(book.cover,
               Some(("cover".to_string(), PathBuf::from("cover.png"), EPubContentType::File(tf.join("cover.png").canonicalize().unwrap()))));
    assert!(buf.is_empty());
}

#[test]
fn nonexistant() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-no-verbose-nonexistant");

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-no-verbose-nonexistant/".to_string(), tf.clone()),
                                    false,
                                    &mut buf),
               Err(Error::FileNotFound {
                   who: "Cover",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(book.cover,
               Some(("cover".to_string(), PathBuf::from("cover.png"), EPubContentType::File(PathBuf::from("cover.png")))));
    assert!(buf.is_empty());
}

#[test]
fn bad_type() {
    let tf = make_test_folder("no-verbose-bad-type");
    let _ = fs::create_dir(tf.join("cover.png"));

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-no-verbose-bad-type/".to_string(), tf.clone()),
                                    false,
                                    &mut buf),
               Err(Error::WrongFileState {
                   what: "a file",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(book.cover,
               Some(("cover".to_string(), PathBuf::from("cover.png"), EPubContentType::File(PathBuf::from("cover.png")))));
    assert!(buf.is_empty());
}
