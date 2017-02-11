use chrono::{DateTime, FixedOffset};
use self::super::super::Error;
use self::super::BookElement;
use std::iter::IntoIterator;
use std::path::PathBuf;
use url::Url;


/// Full ePub book, bundled together.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EPubBook {
    /// E-book's title
    pub name: String,
    /// E-book's author
    pub author: String,
    /// E-book's authoring/publishing date
    pub date: DateTime<FixedOffset>,
    /// Language used in e-book
    pub language: String,
    /// Image to use as e-book cover, if any
    pub cover: Option<EPubCover>,
    /// Content to put in the e-book
    pub content: Vec<EPubContent>,
}

/// All supported variants of content
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EPubContent {
    /// Path to (X)HTML file to use as content
    ///
    /// The content is additionally parsed in search for the text chunk containing `<!-- ePub title: "TOC_NAME" -->`,
    /// where `TOC_NAME` is any string not containing the `"` character.
    ///
    /// That string will be used as the TOC name of the content,
    /// which will allow users on e-book readers to jump directly to the content
    /// represented by the document containing this entry.
    File(PathBuf),
    /// (X)HTML string to use as content
    String(String),
    /// Path to image to include in e-book
    Image(PathBuf),
    /// URL of image to include in e-book
    NetworkImage(Url),
}

/// All supported variants of cover
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EPubCover {
    /// Path to image to use as e-book cover
    File(PathBuf),
    /// URL of image to use as e-book cover
    Network(Url),
}


impl EPubBook {
    /// Construct a book from loose elements
    ///
    /// Returns an error upon violating any of the requirements laid forth in the variants of `BookElement`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate gen_epub_book;
    /// # extern crate chrono;
    /// # fn main() {
    /// # use self::gen_epub_book::ops::{BookElement, EPubContent, EPubCover, EPubBook};
    /// # use self::chrono::DateTime;
    /// # use std::path::PathBuf;
    /// assert_eq!(EPubBook::from_elements(vec![
    ///     BookElement::Name("Simple ePub demonstration".to_string()),
    ///     BookElement::Cover(PathBuf::from("examples/cover.png")),
    ///     BookElement::ImageContent(PathBuf::from("examples/simple/chapter_image.png")),
    ///     BookElement::Content(PathBuf::from("examples/simple/ctnt.html")),
    ///     BookElement::Author("nabijaczleweli".to_string()),
    ///     BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
    ///     BookElement::Language("en-GB".to_string()),
    /// ]), Ok(EPubBook {
    ///     name: "Simple ePub demonstration".to_string(),
    ///     author: "nabijaczleweli".to_string(),
    ///     date: DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap(),
    ///     language: "en-GB".to_string(),
    ///     cover: Some(EPubCover::File(PathBuf::from("examples/cover.png"))),
    ///     content: vec![
    ///         EPubContent::Image(PathBuf::from("examples/simple/chapter_image.png")),
    ///         EPubContent::File(PathBuf::from("examples/simple/ctnt.html")),
    ///     ],
    /// }));
    /// # }
    /// ```
    pub fn from_elements<E: IntoIterator<Item = BookElement>>(elems: E) -> Result<EPubBook, Error> {
        let mut name = None;
        let mut author = None;
        let mut date = None;
        let mut language = None;
        let mut cover = None;
        let mut content = vec![];

        for elem in elems.into_iter() {
            match elem {
                BookElement::Name(n) => name = try!(EPubBook::handle_essential_element(name, n, "Name")),
                BookElement::Content(c) => content.push(EPubContent::File(c)),
                BookElement::StringContent(c) => content.push(EPubContent::String(c)),
                BookElement::ImageContent(c) => content.push(EPubContent::Image(c)),
                BookElement::NetworkImageContent(c) => content.push(EPubContent::NetworkImage(c)),
                BookElement::Cover(c) => cover = try!(EPubBook::handle_essential_element(cover, EPubCover::File(c), "Cover and Network-Cover")),
                BookElement::NetworkCover(c) => cover = try!(EPubBook::handle_essential_element(cover, EPubCover::Network(c), "Cover and Network-Cover")),
                BookElement::Author(a) => author = try!(EPubBook::handle_essential_element(author, a, "Author")),
                BookElement::Date(d) => date = try!(EPubBook::handle_essential_element(date, d, "Date")),
                BookElement::Language(l) => language = try!(EPubBook::handle_essential_element(language, l, "Language")),
            }
        }

        Ok(EPubBook {
            name: try!(EPubBook::require_essential_element(name, "Name")),
            author: try!(EPubBook::require_essential_element(author, "Author")),
            date: try!(EPubBook::require_essential_element(date, "Date")),
            language: try!(EPubBook::require_essential_element(language, "Language")),
            cover: cover,
            content: content,
        })
    }
}

impl EPubBook {
    fn handle_essential_element<T>(el: Option<T>, newval: T, name: &'static str) -> Result<Option<T>, Error> {
        if el.is_some() {
            Err(Error::WrongElementAmount {
                element: name,
                actual: 2,
                relation: "exactly",
                bound: 1,
            })
        } else {
            Ok(Some(newval))
        }
    }

    fn require_essential_element<T>(el: Option<T>, name: &'static str) -> Result<T, Error> {
        if let Some(el) = el {
            Ok(el)
        } else {
            Err(Error::RequiredElementMissing(name))
        }
    }
}
