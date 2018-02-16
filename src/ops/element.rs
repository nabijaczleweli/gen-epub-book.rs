use url::Url;
use std::fmt;
use std::path::PathBuf;
use self::super::super::Error;
use chrono::{DateTime, FixedOffset};


/// A single element of the e-book
///
/// Parse a single line with `BookElement::parse()` or the whole descriptor with `ops::parse_descriptor()`.
///
/// Use `Display` to desugar back to description form.
///
/// # Examples
///
/// ```
/// # use gen_epub_book::ops::BookElement;
/// let input = "Image-Content: images/ch01.png";
/// assert_eq!(&BookElement::parse(input, ":", false).unwrap().unwrap().to_string(), input);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BookElement {
    /// E-book's title
    ///
    /// Required: yes<br />
    /// Type: plaintext<br />
    /// Amount: 1
    Name(String),
    /// Content to put in the e-book
    ///
    /// The content is additionally parsed in search for the text chunk containing `<!-- ePub title: "TOC_NAME" -->`,
    /// where `TOC_NAME` is any string not containing the `"` character.
    ///
    /// That string will be used as the TOC name of the content,
    /// which will allow users on e-book readers to jump directly to the content
    /// represented by the document containing this entry.
    ///
    /// Required: no<br />
    /// Value: relative path to (X)HTML chunk<br />
    /// Amount: any
    Content(PathBuf),
    /// (X)HTML string to use as content
    ///
    /// Required: no<br />
    /// Value: (X)HTML string<br />
    /// Amount: any
    StringContent(String),
    /// Relative path to image to include in e-book
    ///
    /// Required: no<br />
    /// Type: file path<br />
    /// Amount: any
    ImageContent(PathBuf),
    /// URL of image to include in e-book
    ///
    /// Required: no<br />
    /// Type: file URL<br />
    /// Amount: any
    NetworkImageContent(Url),
    /// Relative path to image to use as e-book cover
    ///
    /// Required: no<br />
    /// Type: file path<br />
    /// Amount: 0-1<br />
    /// Remarks: exclusive with Network-Cover
    Cover(PathBuf),
    /// URL of image to use as e-book cover
    ///
    /// Required: no<br />
    /// Type: file URL<br />
    /// Amount: 0-1<br />
    /// Remarks: exclusive with Cover
    NetworkCover(Url),
    /// Auxilliary file to include in e-book
    ///
    /// This is useful for, e.g., CSS.
    ///
    /// Required: no<br />
    /// Value: relative path to (X)HTML chunk<br />
    /// Amount: any
    Include(PathBuf),
    /// URL of auxilliary file to include in e-book
    ///
    /// This is useful for, e.g., fonts.
    ///
    /// Required: no<br />
    /// Type: file URL<br />
    /// Amount: any
    NetworkInclude(Url),
    /// Relative path to file containing the book's description
    ///
    /// Required: no<br />
    /// Value: relative path to (X)HTML chunk<br />
    /// Amount: 0-1<br />
    /// Remarks: exclusive with String-Description and Network-Description
    Description(PathBuf),
    /// Book's description
    ///
    /// Required: no<br />
    /// Type: (X-)HTML<br />
    /// Amount: 0-1<br />
    /// Remarks: exclusive with Description and Network-Description
    StringDescription(String),
    /// URL of auxilliary file containing the book's description
    ///
    /// Required: no<br />
    /// Type: file URL<br />
    /// Amount: 0-1<br />
    /// Remarks: exclusive with Description and String-Description
    NetworkDescription(Url),
    /// E-book's author
    ///
    /// Required: yes<br />
    /// Type: plaintext string<br />
    /// Amount: 1
    Author(String),
    /// E-book's authoring/publishing date
    ///
    /// Required: yes<br />
    /// Type: [RFC3339](https://tools.ietf.org/html/rfc3339)-compliant date<br />
    /// Amount: 1
    Date(DateTime<FixedOffset>),
    /// Language used in e-book
    ///
    /// Required: yes<br />
    /// Type: [BCP47](https://tools.ietf.org/html/bcp47)-compliant language code<br />
    /// Amount: 1
    Language(String),
}

impl BookElement {
    /// (Hopefully) get a book element from a descriptor line with a specified
    /// [separator](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-custom-separator)
    /// with the specified
    /// [rigidness](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-free-date-format).
    ///
    /// If the line isn't a descripting line or the line uses an unknown key, `Ok(None)` is returned.
    ///
    /// `Err` will only be returned when parsing a `DateTime` or a `Url` fails.
    ///
    /// If `free_date` is `true`, in addition to the default RFC3339,
    /// RFC2822 and Unix-timestamp+tt:zz are accepted as correct `DateTime` formats.
    ///
    /// Any whitespace from both parts of the description line are stripped.
    ///
    /// # Examples
    ///
    /// Incorrect format:
    ///
    /// ```
    /// # use gen_epub_book::ops::BookElement;
    /// assert!(BookElement::parse("Date: Mon, 26 Dec 2016 02:01:20 +0100", ":", false).is_err());
    /// assert!(BookElement::parse("Date: 1486564218", ":", true).is_err());
    /// assert!(BookElement::parse("Network-Image-Content: http/i.imgur.com/ViQ2WED.jpg",
    ///                            ":", false).is_err());
    /// ```
    ///
    /// Not a description/unrecognised key:
    ///
    /// ```
    /// # use gen_epub_book::ops::BookElement;
    /// assert_eq!(BookElement::parse("# comment", ":", false), Ok(None));
    /// assert_eq!(BookElement::parse("NetworkImage_Content: that was a typo", ":", false), Ok(None));
    /// assert_eq!(BookElement::parse("Content: used colon instead of equal sign ->", "=", false),
    ///            Ok(None));
    /// assert_eq!(BookElement::parse("Workers all over the world, unite!", ":", false), Ok(None));
    /// ```
    ///
    /// Correct:
    ///
    /// ```
    /// # extern crate gen_epub_book;
    /// # extern crate chrono;
    /// # fn main() {
    /// # use self::gen_epub_book::ops::BookElement;
    /// # use self::chrono::DateTime;
    /// assert_eq!(BookElement::parse("Name: nabijaczleweli", ":", false),
    ///            Ok(Some(BookElement::Name("nabijaczleweli".to_string()))));
    /// assert_eq!(BookElement::parse("Date = 2017-02-08T15:30:18+01:00", "=", false),
    ///            Ok(Some(BookElement::Date(
    ///              DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()))));
    /// assert_eq!(BookElement::parse("Date = Wed, 08 Feb 2017 15:30:18 +0100", "=", true),
    ///            Ok(Some(BookElement::Date(
    ///              DateTime::parse_from_rfc2822("Wed, 08 Feb 2017 15:30:18 +0100").unwrap()))));
    /// assert_eq!(BookElement::parse("Language INCREDIBLE COMMUNISM pl", "INCREDIBLE COMMUNISM", false),
    ///            Ok(Some(BookElement::Language("pl".to_string()))));
    /// # }
    /// ```
    pub fn parse(line: &str, separator: &str, free_date: bool) -> Result<Option<BookElement>, Error> {
        assert!(!separator.is_empty());

        let line = line.trim();
        match line.find(separator) {
            Some(i) => {
                if i == line.len() - 1 {
                    Ok(None)
                } else {
                    let ctnt = line[i + separator.len()..].trim();
                    match line[0..i].trim() {
                        "Name" => Ok(Some(BookElement::Name(ctnt.to_string()))),
                        "Content" => Ok(Some(BookElement::Content(PathBuf::from(ctnt)))),
                        "String-Content" => Ok(Some(BookElement::StringContent(ctnt.to_string()))),
                        "Image-Content" => Ok(Some(BookElement::ImageContent(PathBuf::from(ctnt)))),
                        "Network-Image-Content" => Ok(Some(BookElement::NetworkImageContent(try!(BookElement::parse_url(ctnt))))),
                        "Cover" => Ok(Some(BookElement::Cover(PathBuf::from(ctnt)))),
                        "Network-Cover" => Ok(Some(BookElement::NetworkCover(try!(BookElement::parse_url(ctnt))))),
                        "Include" => Ok(Some(BookElement::Include(PathBuf::from(ctnt)))),
                        "Network-Include" => Ok(Some(BookElement::NetworkInclude(try!(BookElement::parse_url(ctnt))))),
                        "Description" => Ok(Some(BookElement::Description(PathBuf::from(ctnt)))),
                        "String-Description" => Ok(Some(BookElement::StringDescription(ctnt.to_string()))),
                        "Network-Description" => Ok(Some(BookElement::NetworkDescription(try!(BookElement::parse_url(ctnt))))),
                        "Author" => Ok(Some(BookElement::Author(ctnt.to_string()))),
                        "Date" => Ok(Some(BookElement::Date(try!(BookElement::parse_datetime(ctnt, free_date))))),
                        "Language" => Ok(Some(BookElement::Language(ctnt.to_string()))),
                        _ => Ok(None),
                    }
                }
            }
            None => Ok(None),
        }
    }

    /// Get the descriptor name of this element.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate gen_epub_book;
    /// # extern crate url;
    /// # fn main() {
    /// # use self::gen_epub_book::ops::BookElement;
    /// # use std::path::PathBuf;
    /// # use self::url::Url;
    /// assert_eq!(BookElement::Name("nabijaczleweli".to_string()).name(), "Name");
    /// assert_eq!(BookElement::Content(PathBuf::from("content/ch01.html")).name(), "Content");
    /// assert_eq!(BookElement::NetworkImageContent(
    ///                Url::parse("http://i.imgur.com/ViQ2WED.jpg").unwrap()).name(),
    ///            "Network-Image-Content");
    /// # }
    /// ```
    pub fn name(&self) -> &'static str {
        match *self {
            BookElement::Name(_) => "Name",
            BookElement::Content(_) => "Content",
            BookElement::StringContent(_) => "String-Content",
            BookElement::ImageContent(_) => "Image-Content",
            BookElement::NetworkImageContent(_) => "Network-Image-Content",
            BookElement::Cover(_) => "Cover",
            BookElement::NetworkCover(_) => "Network-Cover",
            BookElement::Include(_) => "Include",
            BookElement::NetworkInclude(_) => "Network-Include",
            BookElement::Description(_) => "Description",
            BookElement::StringDescription(_) => "String-Description",
            BookElement::NetworkDescription(_) => "Network-Description",
            BookElement::Author(_) => "Author",
            BookElement::Date(_) => "Date",
            BookElement::Language(_) => "Language",
        }
    }
}

/// Format the element in a way that would make it `parse()`able again with the default separator without Free Date Format.
impl fmt::Display for BookElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}: ", self.name()));
        match *self {
            BookElement::Name(ref s) |
            BookElement::StringContent(ref s) |
            BookElement::StringDescription(ref s) |
            BookElement::Author(ref s) |
            BookElement::Language(ref s) => write!(f, "{}", s),
            BookElement::Content(ref pb) |
            BookElement::ImageContent(ref pb) |
            BookElement::Cover(ref pb) |
            BookElement::Include(ref pb) |
            BookElement::Description(ref pb) => write!(f, "{}", pb.display()),
            BookElement::NetworkImageContent(ref u) |
            BookElement::NetworkCover(ref u) |
            BookElement::NetworkInclude(ref u) |
            BookElement::NetworkDescription(ref u) => write!(f, "{}", u.as_str()),
            BookElement::Date(ref d) => write!(f, "{}", d.to_rfc3339()),
        }
    }
}

impl BookElement {
    fn parse_url(data: &str) -> Result<Url, Error> {
        Url::parse(data).map_err(|_| {
            Error::Parse {
                tp: "URL",
                wher: "book element",
                more: None,
            }
        })
    }

    fn parse_datetime(data: &str, free_date: bool) -> Result<DateTime<FixedOffset>, Error> {
        let dt = DateTime::parse_from_rfc3339(data);
        if free_date {
                dt.or_else(|_| DateTime::parse_from_rfc2822(data))
                    .or_else(|_| DateTime::parse_from_str(data, "%s%:z"))
            } else {
                dt
            }
            .map_err(|_| {
                Error::Parse {
                    tp: "datetime",
                    wher: "book element",
                    more: Some(if free_date {
                        "not RFC3339, RFC2822, nor Unix timestamp w/timezone"
                    } else {
                        "not RFC3339"
                    }),
                }
            })
    }
}
