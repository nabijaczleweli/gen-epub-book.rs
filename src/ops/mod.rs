//! Main functions doing actual work.
//!
//! First, use `parse_descriptor()` to get an element list from a descriptor file,
//! then construct an `EPubBook::from_elements()`, validate and absolutise paths via
//! `EPubBook::normalise_paths()` and write the book with `EPubBook::write_zip()`.


mod book;
mod element;
mod include_dir;

use regex::Regex;
use std::path::Path;
use self::super::Error;
use std::iter::FromIterator;
use std::io::{BufReader, BufRead, Read};

pub use self::element::BookElement;
pub use self::include_dir::IncludeDirectory;
pub use self::book::{EPubContentType, EPubData, EPubBook};


/// Parse the whole descriptor  with a specified
/// [separator](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-custom-separator)
/// with the specified
/// [rigidness](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-free-date-format),
/// stopping at the first encountered error.
///
/// Uses `BookElement::parse()`, so it inherits all errors from there,
/// adding only the error from splitting lines.
///
/// Filters out non-describing lines.
///
/// # Examples
///
/// ```
/// # extern crate gen_epub_book;
/// # extern crate chrono;
/// # fn main() {
/// # use chrono::DateTime;
/// # use std::path::PathBuf;
/// # use gen_epub_book::ops::{BookElement, parse_descriptor};
/// assert_eq!(parse_descriptor("string input", &mut &b"\
///         This is a very simple thing that should prove unproblematic to parse\n\
///         \n\
///         Name: Simple ePub demonstration\n\
///         Cover: cover.png\n\
///         \n\
///         Image-Content: simple/chapter_image.png\n\
///         Content: simple/ctnt.html\n\
///         \n\
///         Author: nabijaczleweli\n\
///         Date: 2017-02-08T15:30:18+01:00\n\
///         Language: en-GB\n"[..], ":", false),
///     Ok(vec![
///         BookElement::Name("Simple ePub demonstration".to_string()),
///         BookElement::Cover(PathBuf::from("cover.png")),
///         BookElement::ImageContent(PathBuf::from("simple/chapter_image.png")),
///         BookElement::Content(PathBuf::from("simple/ctnt.html")),
///         BookElement::Author("nabijaczleweli".to_string()),
///         BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
///         BookElement::Language("en-GB".to_string())]));
/// # }
/// ```
pub fn parse_descriptor<R: Read>(desc: &'static str, from: &mut R, separator: &str, free_date: bool) -> Result<Vec<BookElement>, Error> {
    let elems: Vec<Option<BookElement>> = try!(Result::from_iter(BufReader::new(from)
        .lines()
        .map(|r| {
            r.map_err(|_| {
                Error::Io {
                    desc: desc,
                    op: "read",
                    more: Some("line split"),
                }
            })
        })
        .map(|r| r.and_then(|l| BookElement::parse(&l, separator, free_date)))
        .collect::<Vec<_>>()));

    Ok(elems.into_iter().flat_map(|o| o).collect())
}

/// Find an ePub title line in the specified input stream.
///
/// The title line contains `<!-- ePub title: "TOC_NAME" -->`, where `TOC_NAME` is any string not containing the `"` character.
///
/// # Examples
///
///
///
///
/// ```
/// # use gen_epub_book::ops::find_title;
/// assert_eq!(find_title(&mut &br#"L1\nL <!-- ePub title: "TTL" -->2\nL3"#[..]),
///            Some("TTL".to_string()));
/// ```
pub fn find_title<R: Read>(i: &mut R) -> Option<String> {
    lazy_static! {
        static ref TITLE_RGX: Regex = Regex::new(r#"<!-- ePub title: "([^"]+)" -->"#).unwrap();
    }

    BufReader::new(i)
        .lines()
        .find(|l| if l.is_ok() {
            TITLE_RGX.is_match(l.as_ref().unwrap())
        } else {
            false
        })
        .map(|l| TITLE_RGX.captures(&l.unwrap()).unwrap().get(1).unwrap().as_str().to_string())
}

/// Find an appropriate `IncludeDirectory` for the specified file, or `None` otherwise
///
/// # Examples
///
/// Given:
///
/// ```text
/// special_book
/// ├── rendered
/// │   └── output
/// │       ├── intro.html
/// │       ├── main.html
/// │       └── ending.html
/// ├── previews
/// │   └── generated
/// │       └── out
/// │           ├── intro.html
/// │           └── main.html
/// └── gep
///    └── special
///        ├── intro.html
///        └── book.epupp
/// ```
///
/// The following holds:
///
/// ```
/// # use gen_epub_book::ops::{IncludeDirectory, find_file};
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use std::path::Path;
/// # let special_book = temp_dir().join("gen-epub-book.rs-doctest").join("ops-find-file-0").join("special_book");
/// # fs::create_dir_all(special_book.join("rendered").join("output")).unwrap();
/// # fs::create_dir_all(special_book.join("previews").join("generated").join("out")).unwrap();
/// # fs::create_dir_all(special_book.join("gep").join("special")).unwrap();
/// # for f in &["intro.html", "main.html", "ending.html"] {
/// #   File::create(special_book.join("rendered").join("output").join(f)).unwrap();
/// # }
/// # for f in &["intro.html", "main.html"] {
/// #   File::create(special_book.join("previews").join("generated").join("out").join(f)).unwrap();
/// # }
/// # for f in &["intro.html", "book.epupp"] {
/// #   File::create(special_book.join("gep").join("special").join(f)).unwrap();
/// # }
/// let default = IncludeDirectory::Unnamed {
///     dir: ("".to_string(),
///           special_book.join("gep").join("special")),
/// };
/// let previews = IncludeDirectory::Named {
///     name: "previews".to_string(),
///     dir: ("../../previews/generated/out".to_string(),
///           special_book.join("previews").join("generated").join("out")),
/// };
/// let rendered = IncludeDirectory::Unnamed {
///     dir: ("../../rendered/output".to_string(),
///           special_book.join("rendered").join("output")),
/// };
///
/// let include_order = [default.clone(), previews.clone(), rendered.clone()];
/// assert_eq!(find_file(&Path::new("intro.html"), &include_order), Some(&default));
/// assert_eq!(find_file(&Path::new("main.html"), &include_order), Some(&previews));
/// assert_eq!(find_file(&Path::new("ending.html"), &include_order), Some(&rendered));
/// assert_eq!(find_file(&Path::new("cover.png"), &include_order), None);
/// ```
pub fn find_file<P: AsRef<Path>>(file: P, include_order: &[IncludeDirectory]) -> Option<&IncludeDirectory> {
    let file = file.as_ref();
    include_order.iter().find(|i| i.resolve(file).is_some())
}
