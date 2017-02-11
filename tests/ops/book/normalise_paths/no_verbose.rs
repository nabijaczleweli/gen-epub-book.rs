use gen_epub_book::ops::{EPubContent, EPubCover, EPubBook};
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use std::env::temp_dir;
use chrono::DateTime;


#[test]
fn correct() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-no-verbose-correct");
    let _ = fs::create_dir_all(tf.join("content"));
    File::create(tf.join("cover.png")).unwrap();
    File::create(tf.join("content").join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook {
        name: "".to_string(),
        author: "".to_string(),
        date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
        language: "".to_string(),
        cover: Some(EPubCover::File(("cover".to_string(), PathBuf::from("cover.png")))),
        content: vec![EPubContent::File(("content-ch01".to_string(), PathBuf::from("content/ch01.html")))],
    };

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-no-verbose-correct/".to_string(), tf.clone()), false, &mut buf),
               Ok(()));
    assert_eq!(book,
               EPubBook {
                   name: "".to_string(),
                   author: "".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "".to_string(),
                   cover: Some(EPubCover::File(("cover".to_string(), tf.join("cover.png").canonicalize().unwrap()))),
                   content: vec![EPubContent::File(("content-ch01".to_string(), tf.join("content").join("ch01.html").canonicalize().unwrap()))],
               });
    assert!(buf.is_empty());
}

#[test]
fn nonexistant() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-no-verbose-nonexistant");

    let mut buf = vec![];
    let mut book = EPubBook {
        name: "".to_string(),
        author: "".to_string(),
        date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
        language: "".to_string(),
        cover: None,
        content: vec![EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
    };

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-no-verbose-nonexistant/".to_string(), tf.clone()),
                                    false,
                                    &mut buf),
               Err(Error::FileNotFound {
                   who: "Image-Content",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(book,
               EPubBook {
                   name: "".to_string(),
                   author: "".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "".to_string(),
                   cover: None,
                   content: vec![EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
               });
    assert!(buf.is_empty());
}

#[test]
fn bad_type() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-no-verbose-bad-type");
    let _ = fs::create_dir_all(tf.join("cover.png"));

    let mut buf = vec![];
    let mut book = EPubBook {
        name: "".to_string(),
        author: "".to_string(),
        date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
        language: "".to_string(),
        cover: None,
        content: vec![EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
    };

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-no-verbose-bad-type/".to_string(), tf.clone()),
                                    false,
                                    &mut buf),
               Err(Error::WrongFileState {
                   what: "a file",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(book,
               EPubBook {
                   name: "".to_string(),
                   author: "".to_string(),
                   date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
                   language: "".to_string(),
                   cover: None,
                   content: vec![EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
               });
    assert!(buf.is_empty());
}
