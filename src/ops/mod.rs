mod element;

use std::path::PathBuf;
use self::super::Error;
use std::iter::FromIterator;
use std::io::{BufReader, BufRead, Write, Read};

pub use self::element::BookElement;


/// Parse the whole descriptor, stopping at the first encountered error
///
/// Uses `BookElement::parse()`, so it inherits all errors from there,
/// adding only the error from splitting lines.
///
/// Flattens out non-describing lines.
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
///         Language: en-GB\n"[..]),
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
pub fn parse_descriptor<R: Read>(desc: &'static str, from: &mut R) -> Result<Vec<BookElement>, Error> {
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
        .map(|r| r.and_then(|l| BookElement::parse(&l)))
        .collect::<Vec<_>>()));

    Ok(elems.into_iter().flat_map(|o| o).collect())
}

/// Normalise the paths in the specified book elements based on the specified relative path root,
/// optionally printing verbose output to the specified stream.
///
/// Will return an error if the file the path points to doesn't exist or isn't a file.
///
/// # Examples
///
/// ```
/// # use gen_epub_book::ops::{BookElement, normalise_paths};
/// # use std::fs::{self, File};
/// # use std::path::PathBuf;
/// # use std::env::temp_dir;
/// # use std::io::stdout;
/// # let tf = temp_dir().join("gen-epub-book.rs-doctest").join("ops-normalise-paths-0");
/// # fs::create_dir_all(&tf).unwrap();
/// # File::create(tf.join("cover.png")).unwrap();
/// # let mut elems = [BookElement::Cover(PathBuf::from("cover.png"))];
/// # /*
/// normalise_paths(&mut elems, &("./".to_string(), PathBuf::new()),
///                 false, &mut stdout()).unwrap();
/// assert_eq!(elems[0], BookElement::Cover(PathBuf::from("cover.png").canonicalize().unwrap()));
/// # */
/// # normalise_paths(&mut elems, &("$TEMP/ops-normalise-paths-0/".to_string(), tf.clone()), false, &mut stdout()).unwrap();
/// # assert_eq!(elems[0], BookElement::Cover(tf.join("cover.png").canonicalize().unwrap()));
/// ```
pub fn normalise_paths<W: Write>(elems: &mut [BookElement], relroot: &(String, PathBuf), verbose: bool, verb_out: &mut W) -> Result<(), Error> {
    for elem in elems {
        let name = elem.name();
        match elem {
            &mut BookElement::Content(ref mut pb) |
            &mut BookElement::ImageContent(ref mut pb) |
            &mut BookElement::Cover(ref mut pb) => {
                let new = relroot.1.join(&pb);
                if !new.exists() {
                    return Err(Error::FileNotFound {
                        who: name,
                        path: new,
                    });
                } else if !new.is_file() {
                    return Err(Error::WrongFileState {
                        what: "a file",
                        path: new,
                    });
                } else {
                    if verbose {
                        writeln!(verb_out, "Normalised {} to {}{} for {}.", pb.display(), relroot.0, pb.display(), name).unwrap();
                    }
                    *pb = new.canonicalize().unwrap();
                }
            }
            _ => {}
        }
    }

    Ok(())
}
