use gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
use self::super::make_test_folder;
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use chrono::DateTime;


#[test]
fn correct() {
    let tf = make_test_folder("verbose-correct");
    let _ = fs::create_dir(tf.join("content"));
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
    let tf = make_test_folder("verbose-nonexistant");
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
    let tf = make_test_folder("verbose-bad-type");
    let _ = fs::create_dir(tf.join("ch01.html"));
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
