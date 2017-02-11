use gen_epub_book::ops::{BookElement, normalise_paths};
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use std::env::temp_dir;


#[test]
fn correct() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-normalise-paths-no-verbose-correct");
    fs::create_dir_all(tf.join("content")).unwrap();
    File::create(tf.join("cover.png")).unwrap();
    File::create(tf.join("content").join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut elems = [BookElement::Cover(PathBuf::from("cover.png")), BookElement::Content(PathBuf::from("content/ch01.html"))];

    assert_eq!(normalise_paths(&mut elems, &("$TEMP/ops-normalise-paths-no-verbose-correct/".to_string(), tf.clone()), false, &mut buf),
               Ok(()));
    assert_eq!(elems,
               [BookElement::Cover(tf.join("cover.png").canonicalize().unwrap()),
                BookElement::Content(tf.join("content").join("ch01.html").canonicalize().unwrap())]);
    assert!(buf.is_empty());
}

#[test]
fn nonexistant() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-normalise-paths-no-verbose-nonexistant");

    let mut buf = vec![];
    let mut elems = [BookElement::ImageContent(PathBuf::from("cover.png"))];

    assert_eq!(normalise_paths(&mut elems, &("$TEMP/ops-normalise-paths-no-verbose-nonexistant/".to_string(), tf.clone()), false, &mut buf),
               Err(Error::FileNotFound {
                   who: "Image-Content",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(elems, [BookElement::ImageContent(PathBuf::from("cover.png"))]);
    assert!(buf.is_empty());
}

#[test]
fn bad_type() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-normalise-paths-no-verbose-bad-type");
    fs::create_dir_all(tf.join("cover.png")).unwrap();

    let mut buf = vec![];
    let mut elems = [BookElement::ImageContent(PathBuf::from("cover.png"))];

    assert_eq!(normalise_paths(&mut elems, &("$TEMP/ops-normalise-paths-no-verbose-bad-type/".to_string(), tf.clone()), false, &mut buf),
               Err(Error::WrongFileState {
                   what: "a file",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(elems, [BookElement::ImageContent(PathBuf::from("cover.png"))]);
    assert!(buf.is_empty());
}
