use gen_epub_book::ops::{BookElement, normalise_paths};
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use std::env::temp_dir;


#[test]
fn correct() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-normalise-paths-verbose-correct");
    fs::create_dir_all(tf.join("content")).unwrap();
    File::create(tf.join("cover.png")).unwrap();
    File::create(tf.join("content").join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut elems = [BookElement::Cover(PathBuf::from("cover.png")), BookElement::Content(PathBuf::from("content/ch01.html"))];

    assert_eq!(normalise_paths(&mut elems,
                               &("$TEMP/ops-normalise-paths-verbose-correct/".to_string(), tf.clone()),
                               true,
                               &mut buf),
               Ok(()));
    assert_eq!(elems,
               [BookElement::Cover(tf.join("cover.png").canonicalize().unwrap()),
                BookElement::Content(tf.join("content").join("ch01.html").canonicalize().unwrap())]);
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised cover.png to $TEMP/ops-normalise-paths-verbose-correct/cover.png for Cover.\n\
                Normalised content/ch01.html to $TEMP/ops-normalise-paths-verbose-correct/content/ch01.html for Content.\n");
}

#[test]
fn nonexistant() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-normalise-paths-verbose-nonexistant");
    fs::create_dir_all(&tf).unwrap();
    File::create(tf.join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut elems = [BookElement::Content(PathBuf::from("ch01.html")), BookElement::ImageContent(PathBuf::from("cover.png"))];

    assert_eq!(normalise_paths(&mut elems,
                               &("$TEMP/ops-normalise-paths-verbose-nonexistant/".to_string(), tf.clone()),
                               true,
                               &mut buf),
               Err(Error::FileNotFound {
                   who: "Image-Content",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(elems,
               [BookElement::Content(tf.join("ch01.html").canonicalize().unwrap()), BookElement::ImageContent(PathBuf::from("cover.png"))]);
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised ch01.html to $TEMP/ops-normalise-paths-verbose-nonexistant/ch01.html for Content.\n");
}

#[test]
fn bad_type() {
    let tf = temp_dir().join("gen-epub-book.rs-test").join("ops-normalise-paths-verbose-bad-type");
    fs::create_dir_all(tf.join("cover.png")).unwrap();
    File::create(tf.join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut elems = [BookElement::Content(PathBuf::from("ch01.html")), BookElement::ImageContent(PathBuf::from("cover.png"))];

    assert_eq!(normalise_paths(&mut elems,
                               &("$TEMP/ops-normalise-paths-verbose-bad-type/".to_string(), tf.clone()),
                               true,
                               &mut buf),
               Err(Error::WrongFileState {
                   what: "a file",
                   path: tf.join("cover.png"),
               }));
    assert_eq!(elems,
               [BookElement::Content(tf.join("ch01.html").canonicalize().unwrap()), BookElement::ImageContent(PathBuf::from("cover.png"))]);
    assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
               "Normalised ch01.html to $TEMP/ops-normalise-paths-verbose-bad-type/ch01.html for Content.\n");
}
