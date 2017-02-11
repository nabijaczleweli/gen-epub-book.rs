use gen_epub_book::ops::{EPubContent, EPubCover, EPubBook};
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
    let mut book = EPubBook {
        name: "".to_string(),
        author: "".to_string(),
        date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
        language: "".to_string(),
        cover: Some(EPubCover::File(("cover".to_string(), PathBuf::from("cover.png")))),
        content: vec![EPubContent::File(("content-ch01".to_string(), PathBuf::from("content/ch01.html")))],
    };

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-correct/".to_string(), tf.clone()), true, &mut buf),
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
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised cover.png to $TEMP/ops-book-normalise-paths-verbose-correct/cover.png for Cover.\n\
                Normalised content/ch01.html to $TEMP/ops-book-normalise-paths-verbose-correct/content/ch01.html for Content.\n");
}

#[test]
fn nonexistant() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-verbose-nonexistant");
    let _ = fs::create_dir_all(&tf);
    File::create(tf.join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook {
        name: "".to_string(),
        author: "".to_string(),
        date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
        language: "".to_string(),
        cover: None,
        content: vec![EPubContent::File(("ch01".to_string(), PathBuf::from("ch01.html"))),
                      EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
    };

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-nonexistant/".to_string(), tf.clone()), true, &mut buf),
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
                   content: vec![EPubContent::File(("ch01".to_string(), tf.join("ch01.html").canonicalize().unwrap())),
                                 EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
               });
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised ch01.html to $TEMP/ops-book-normalise-paths-verbose-nonexistant/ch01.html for Content.\n");
}

#[test]
fn bad_type() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-book-normalise-paths-verbose-bad-type");
    let _ = fs::create_dir_all(tf.join("cover.png"));
    File::create(tf.join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook {
        name: "".to_string(),
        author: "".to_string(),
        date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
        language: "".to_string(),
        cover: None,
        content: vec![EPubContent::File(("ch01".to_string(), PathBuf::from("ch01.html"))),
                      EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
    };

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-bad-type/".to_string(), tf.clone()), true, &mut buf),
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
                   content: vec![EPubContent::File(("ch01".to_string(), tf.join("ch01.html").canonicalize().unwrap())),
                                 EPubContent::Image(("cover".to_string(), PathBuf::from("cover.png")))],
               });
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised ch01.html to $TEMP/ops-book-normalise-paths-verbose-bad-type/ch01.html for Content.\n");
}
