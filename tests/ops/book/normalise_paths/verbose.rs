use gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use std::env::temp_dir;
use chrono::DateTime;


#[test]
fn correct() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-verbose-correct");
    let _ = fs::create_dir_all(tf.join("content"));
    File::create(tf.join("cover.png")).unwrap();
    File::create(tf.join("content").join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png")),
                                                BookElement::Content(PathBuf::from("content/ch01.html"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-correct/".to_string(), tf.clone()), true, &mut buf),
               Ok(()));
    assert_eq!(book.cover,
               Some(("cover".to_string(), PathBuf::from("cover.png"), EPubContentType::File(tf.join("cover.png").canonicalize().unwrap()))));
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised cover.png to $TEMP/ops-book-normalise-paths-verbose-correct/cover.png for Cover.\n\
                Normalised content/ch01.html to $TEMP/ops-book-normalise-paths-verbose-correct/content/ch01.html for Content or Image.\n");
}

#[test]
fn nonexistant() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-verbose-nonexistant");
    let _ = fs::create_dir_all(&tf);
    File::create(tf.join("cover.png")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png")),
                                                BookElement::Content(PathBuf::from("ch01.html"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-nonexistant/".to_string(), tf.clone()), true, &mut buf),
               Err(Error::FileNotFound {
                   who: "Content or Image",
                   path: tf.join("ch01.html"),
               }));
    assert_eq!(book.cover,
               Some(("cover".to_string(), PathBuf::from("cover.png"), EPubContentType::File(tf.join("cover.png").canonicalize().unwrap()))));
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised cover.png to $TEMP/ops-book-normalise-paths-verbose-nonexistant/cover.png for Cover.\n");
}

#[test]
fn bad_type() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-verbose-bad-type");
    let _ = fs::create_dir_all(tf.join("ch01.html"));
    File::create(tf.join("cover.png")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png")),
                                                BookElement::Content(PathBuf::from("ch01.html"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-bad-type/".to_string(), tf.clone()), true, &mut buf),
               Err(Error::WrongFileState {
                   what: "a file",
                   path: tf.join("ch01.html"),
               }));
    assert_eq!(book.cover,
               Some(("cover".to_string(), PathBuf::from("cover.png"), EPubContentType::File(tf.join("cover.png").canonicalize().unwrap()))));
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised cover.png to $TEMP/ops-book-normalise-paths-verbose-bad-type/cover.png for Cover.\n");
}
