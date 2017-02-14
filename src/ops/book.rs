use self::super::super::util::{CONTENT_TABLE_HEADER, CONTAINER, MIME_TYPE, write_string_content, xhtml_path_id, book_filename, xhtml_url_id, download_to};
use self::super::{BookElement, find_title};
use zip::write::{ZipWriter, FileOptions};
use chrono::{DateTime, FixedOffset};
use mime_guess::guess_mime_type_opt;
use std::io::{self, Write, Seek};
use std::collections::BTreeSet;
use self::super::super::Error;
use std::iter::IntoIterator;
use std::path::PathBuf;
use std::fs::File;
use uuid::Uuid;
use url::Url;


/// An (ID, filename, data) tuple that represents a single data point of an ePub.
pub type EPubData = (String, PathBuf, EPubContentType);

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
    pub cover: Option<EPubData>,
    /// Content to put in the e-book
    content: Vec<EPubData>,
    /// Things that aren't *content* but go in the e-book
    non_content: Vec<EPubData>,
    /// E-book's UUID
    uuid: Uuid,
}

/// Enum representing what can go in an ePub
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EPubContentType {
    /// Sourced from a file
    File(PathBuf),
    /// Sourced from network
    Network(Url),
    /// Sourced from a string
    ///
    /// Wrapped in `util::write_string_content()`.
    Raw(String),
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
    /// # use self::gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
    /// # use self::chrono::DateTime;
    /// # use std::path::PathBuf;
    /// let book = EPubBook::from_elements(vec![
    ///     BookElement::Name("Simple ePub demonstration".to_string()),
    ///     BookElement::Cover(PathBuf::from("examples/cover.png")),
    ///     BookElement::ImageContent(PathBuf::from("examples/simple/chapter_image.png")),
    ///     BookElement::Content(PathBuf::from("examples/simple/ctnt.html")),
    ///     BookElement::Author("nabijaczleweli".to_string()),
    ///     BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
    ///     BookElement::Language("en-GB".to_string()),
    /// ]).unwrap();
    ///
    /// assert_eq!(book.name, "Simple ePub demonstration".to_string());
    /// assert_eq!(book.author, "nabijaczleweli".to_string());
    /// assert_eq!(book.date, DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap());
    /// assert_eq!(book.language, "en-GB".to_string());
    /// assert_eq!(book.cover, Some(("examples-cover".to_string(),
    ///                              PathBuf::from("examples-cover.png"),
    ///                              EPubContentType::File(PathBuf::from("examples/cover.png")))));
    /// # }
    /// ```
    pub fn from_elements<E: IntoIterator<Item = BookElement>>(elems: E) -> Result<EPubBook, Error> {
        let mut name = None;
        let mut author = None;
        let mut date = None;
        let mut language = None;
        let mut cover = None;
        let mut content = vec![];
        let mut non_content = vec![];

        for (i, elem) in elems.into_iter().enumerate() {
            match elem {
                BookElement::Name(n) => name = try!(EPubBook::handle_essential_element(name, n, "Name")),
                BookElement::Content(c) => content.push((xhtml_path_id(&c), book_filename(&c), EPubContentType::File(c))),
                BookElement::StringContent(c) => {
                    content.push((format!("string-content-{}", i), PathBuf::from(format!("string-data-{}.html", i)), EPubContentType::Raw(c)))
                }
                BookElement::ImageContent(c) => {
                    let fname = book_filename(&c);
                    non_content.push((xhtml_path_id(&c), fname.clone(), EPubContentType::File(c)));
                    content.push((format!("image-content-{}", i),
                                  PathBuf::from(format!("image-data-{}.html", i)),
                                  EPubContentType::Raw(format!(r#"<center><img src="{}"></img></center>"#, fname.display()))));
                }
                BookElement::NetworkImageContent(c) => {
                    let fname = c.path_segments().unwrap().last().unwrap().to_string();
                    non_content.push((xhtml_url_id(&c).to_string(), PathBuf::from(&fname), EPubContentType::Network(c)));
                    content.push((format!("network-image-content-{}", i),
                                  PathBuf::from(format!("network-image-data-{}.html", i)),
                                  EPubContentType::Raw(format!(r#"<center><img src="{}"></img></center>"#, fname))));
                }
                BookElement::Cover(c) => {
                    cover = try!(EPubBook::handle_essential_element(cover,
                                                                    (xhtml_path_id(&c), book_filename(&c), EPubContentType::File(c)),
                                                                    "Cover and Network-Cover"))
                }
                BookElement::NetworkCover(c) => {
                    cover = try!(EPubBook::handle_essential_element(cover,
                                                                    (format!("network-cover-{}", xhtml_url_id(&c)),
                                                                     PathBuf::from(c.path_segments().unwrap().last().unwrap()),
                                                                     EPubContentType::Network(c)),
                                                                    "Cover and Network-Cover"))
                }
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
            non_content: non_content,
            uuid: Uuid::new_v4(),
        })
    }

    /// Normalise the paths in the book based on the specified relative path root,
    /// optionally printing verbose output to the specified stream.
    ///
    /// Will return an error if the file the path points to doesn't exist or isn't a file.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate gen_epub_book;
    /// # extern crate chrono;
    /// # fn main() {
    /// # use self::gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
    /// # use self::chrono::DateTime;
    /// # use std::fs::{self, File};
    /// # use std::path::PathBuf;
    /// # use std::env::temp_dir;
    /// # use std::io::stdout;
    /// # let tf = temp_dir().join("gen-epub-book.rs-doctest").join("ops-epub-book-normalise-paths-0");
    /// # fs::create_dir_all(tf.join("content")).unwrap();
    /// # File::create(tf.join("cover.png")).unwrap();
    /// # File::create(tf.join("content").join("ch01.html")).unwrap();
    /// let mut book = EPubBook::from_elements(vec![
    ///     BookElement::Name("Path normalisation demonstration".to_string()),
    ///     BookElement::Cover(PathBuf::from("cover.png")),
    ///     BookElement::Content(PathBuf::from("content/ch01.html")),
    ///     BookElement::Author("nabijaczleweli".to_string()),
    ///     BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
    ///     BookElement::Language("en-GB".to_string()),
    /// ]).unwrap();
    /// # if false {
    /// book.normalise_paths(&("./".to_string(), PathBuf::new()), false, &mut stdout()).unwrap();
    /// assert_eq!(book.cover, Some(("cover".to_string(),
    ///                              PathBuf::from("cover.png"),
    ///                              EPubContentType::File(PathBuf::from("cover.png").canonicalize().unwrap()))));
    /// # }
    /// # book.normalise_paths(&("$TEMP/ops-epub-book-normalise-paths-0/".to_string(), tf.clone()), false, &mut
    /// vec![]).unwrap();
    /// # assert_eq!(book.cover, Some(("cover".to_string(),
    /// #                              PathBuf::from("cover.png"),
    /// #                              EPubContentType::File(tf.join("cover.png").canonicalize().unwrap()))));
    /// # }
    /// ```
    pub fn normalise_paths<W: Write>(&mut self, relroot: &(String, PathBuf), verbose: bool, verb_out: &mut W) -> Result<(), Error> {
        if let Some(&mut (_, _, EPubContentType::File(ref mut c))) = self.cover.as_mut() {
            try!(EPubBook::normalise_path(relroot, c, "Cover", verbose, verb_out));
        }

        for ctnt in self.content.iter_mut().chain(self.non_content.iter_mut()) {
            if let EPubContentType::File(ref mut pb) = ctnt.2 {
                try!(EPubBook::normalise_path(relroot, pb, "Content or Image", verbose, verb_out));
            }
        }

        Ok(())
    }

    /// Write the book as ePub into the specified stream, optionally logging verbose output.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate gen_epub_book;
    /// # extern crate chrono;
    /// # fn main() {
    /// # use self::gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
    /// # use self::chrono::DateTime;
    /// # use std::fs::{self, File};
    /// # use std::path::PathBuf;
    /// # use std::env::temp_dir;
    /// # use std::io::stdout;
    /// # let tf = temp_dir().join("gen-epub-book.rs-doctest").join("ops-epub-book-write-zip-0");
    /// # fs::create_dir_all(tf.join("content")).unwrap();
    /// # File::create(tf.join("cover.png")).unwrap();
    /// # File::create(tf.join("content").join("ch01.html")).unwrap();
    /// let mut book = EPubBook::from_elements(vec![
    ///     BookElement::Name("Path normalisation demonstration".to_string()),
    ///     BookElement::Cover(PathBuf::from("cover.png")),
    ///     BookElement::Content(PathBuf::from("content/ch01.html")),
    ///     BookElement::Author("nabijaczleweli".to_string()),
    ///     BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
    ///     BookElement::Language("en-GB".to_string()),
    /// ]).unwrap();
    /// # if false {
    /// book.normalise_paths(&("./".to_string(), PathBuf::new()), false, &mut stdout()).unwrap();
    /// # }
    /// # book.normalise_paths(&("$TEMP/ops-epub-book-write-zip-0/".to_string(), tf.clone()), false, &mut vec![]).unwrap();
    /// # if false {
    /// book.write_zip(&mut File::create("write_zip.epub").unwrap(), false, &mut stdout()).unwrap();
    /// # }
    /// # book.write_zip(&mut File::create(tf.join("write_zip.epub")).unwrap(), false, &mut vec![]).unwrap();
    /// # assert!(tf.join("write_zip.epub").metadata().unwrap().len() > 0);
    /// # }
    /// ```
    pub fn write_zip<W: Write + Seek, V: Write>(&self, to: &mut W, verbose: bool, verb_out: &mut V) -> Result<(), Error> {
        let mut w = ZipWriter::new(to);

        try!(w.start_file("META-INF/container.xml", FileOptions::default()).map_err(|_| EPubBook::zip_error("create", "container file")));
        try!(w.write_all(CONTAINER.as_bytes()).map_err(|_| EPubBook::zip_error("write", "container file")));

        try!(w.start_file("mimetype", FileOptions::default()).map_err(|_| EPubBook::zip_error("create", "container file")));
        try!(w.write_all(MIME_TYPE.as_bytes()).map_err(|_| EPubBook::zip_error("write", "container file")));

        try!(w.start_file("content.opf", FileOptions::default()).map_err(|_| EPubBook::zip_error("create", "content table")));
        try!(self.content_table(&mut w));

        try!(w.start_file("toc.ncx", FileOptions::default()).map_err(|_| EPubBook::zip_error("create", "table of contents")));
        try!(self.table_of_contents(&mut w, verbose, verb_out));

        try!(self.write_content(&mut w, verbose, verb_out));

        Ok(())
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

    fn normalise_path<W: Write>(relroot: &(String, PathBuf), what: &mut PathBuf, name: &'static str, verbose: bool, verb_out: &mut W) -> Result<(), Error> {
        let new = relroot.1.join(&what);
        if !new.exists() {
            Err(Error::FileNotFound {
                who: name,
                path: new,
            })
        } else if !new.is_file() {
            Err(Error::WrongFileState {
                what: "a file",
                path: new,
            })
        } else {
            if verbose {
                let _ = writeln!(verb_out, "Normalised {} to {}{0} for {}.", what.display(), relroot.0, name);
            }
            *what = new.canonicalize().unwrap();
            Ok(())
        }
    }

    fn zip_error(verb: &'static str, what: &'static str) -> Error {
        Error::Io {
            desc: "ePub",
            op: verb,
            more: Some(what),
        }
    }

    fn content_table<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        try!(w.write_all(CONTENT_TABLE_HEADER.as_bytes()).map_err(|_| EPubBook::zip_error("write", "content table header")));
        try!(writeln!(w, "    <dc:title>{}</dc:title>", self.name).map_err(|_| EPubBook::zip_error("write", "content table title line")));
        try!(writeln!(w, r#"    <dc:creator opf:role="aut">{}</dc:creator>"#, self.author)
            .map_err(|_| EPubBook::zip_error("write", "content table author line")));
        try!(writeln!(w,
                      r#"    <dc:identifier id="uuid" opf:scheme="uuid">{}</dc:identifier>"#,
                      self.uuid.hyphenated())
            .map_err(|_| EPubBook::zip_error("write", "content table uuid line")));
        try!(writeln!(w, r#"    <dc:date>{}</dc:date>"#, self.date.to_rfc3339()).map_err(|_| EPubBook::zip_error("write", "content table date line")));
        try!(writeln!(w, r#"    <dc:language>{}</dc:language>"#, self.language).map_err(|_| EPubBook::zip_error("write", "content table language line")));

        if let Some(&(ref id, _, _)) = self.cover.as_ref() {
            try!(writeln!(w, r#"    <meta name="cover" content="{}" />"#, id).map_err(|_| EPubBook::zip_error("write", "content table cover line")));
        }

        try!(writeln!(w, r#"  </metadata>"#).map_err(|_| EPubBook::zip_error("write", "content table metadata end")));
        try!(writeln!(w, r#"  <manifest>"#).map_err(|_| EPubBook::zip_error("write", "content table manifest start")));
        try!(writeln!(w, r#"    <item href="toc.ncx" id="toc" media-type="application/x-dtbncx+xml"/>"#)
            .map_err(|_| EPubBook::zip_error("write", "content table manifest toc line")));

        let mut specified_ids = BTreeSet::new();
        for &(ref id, ref fname, _) in self.cover.iter().chain(self.content.iter()).chain(self.non_content.iter()) {
            if !specified_ids.contains(&id[..]) {
                specified_ids.insert(&id[..]);
                try!(writeln!(w,
                              r#"    <item href="{}" id="{}" media-type="{}" />"#,
                              fname.display(),
                              id,
                              try!(guess_mime_type_opt(&fname).ok_or(Error::WrongFileState {
                                  what: "of recognised extension",
                                  path: fname.clone(),
                              })))
                    .map_err(|_| EPubBook::zip_error("write", "content table manifest content")));
            }
        }

        try!(writeln!(w, r#"  </manifest>"#).map_err(|_| EPubBook::zip_error("write", "content table manifest end")));
        try!(writeln!(w, r#"  <spine toc="toc">"#).map_err(|_| EPubBook::zip_error("write", "content table spine start")));

        for &(ref id, _, _) in self.content.iter() {
            try!(writeln!(w, r#"    <itemref idref="{}" />"#, id).map_err(|_| EPubBook::zip_error("write", "content table spine content")));
        }

        try!(writeln!(w, r#"  </spine>"#).map_err(|_| EPubBook::zip_error("write", "content table spine end")));
        try!(writeln!(w, r#"  <guide>"#).map_err(|_| EPubBook::zip_error("write", "content table guide start")));

        if let Some(&(ref id, ref fname, _)) = self.cover.as_ref() {
            try!(writeln!(w, r#"    <reference xmlns="http://www.idpf.org/2007/opf" href="{}" title="{}" type="cover" />"#, fname.display(), id)
                .map_err(|_| EPubBook::zip_error("write", "content table guide cover")));
        }

        try!(writeln!(w, r#"    <reference href="toc.ncx" title="Table of Contents" type="toc" />"#)
            .map_err(|_| EPubBook::zip_error("write", "content table guide toc")));
        try!(writeln!(w, r#"  </guide>"#).map_err(|_| EPubBook::zip_error("write", "content table guide end")));
        try!(writeln!(w, r#"</package>"#).map_err(|_| EPubBook::zip_error("write", "content table package end")));

        Ok(())
    }

    fn table_of_contents<W: Write, V: Write>(&self, w: &mut W, verbose: bool, verb_out: &mut V) -> Result<(), Error> {
        try!(writeln!(w, r#"<?xml version='1.0' encoding='utf-8'?>"#).map_err(|_| EPubBook::zip_error("write", "toc xml start")));
        try!(writeln!(w, r#"<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1" xml:lang="{}">"#, self.language)
            .map_err(|_| EPubBook::zip_error("write", "toc ncx start")));
        try!(writeln!(w, r#"  <head>"#).map_err(|_| EPubBook::zip_error("write", "toc head start")));
        try!(writeln!(w, r#"    <meta content="{}" name="dtb:uid"/>"#, self.uuid.hyphenated()).map_err(|_| EPubBook::zip_error("write", "toc head uuid")));
        try!(writeln!(w, r#"    <meta content="2" name="dtb:depth"/>"#).map_err(|_| EPubBook::zip_error("write", "toc head depth")));
        try!(writeln!(w, r#"  </head>"#).map_err(|_| EPubBook::zip_error("write", "toc head end")));
        try!(writeln!(w, r#"  <docTitle>"#).map_err(|_| EPubBook::zip_error("write", "toc doc title start")));
        try!(writeln!(w, r#"    <text>{}</text>"#, self.name).map_err(|_| EPubBook::zip_error("write", "toc doc title")));
        try!(writeln!(w, r#"  </docTitle>"#).map_err(|_| EPubBook::zip_error("write", "toc doc title end")));
        try!(writeln!(w, r#"  <navMap>"#).map_err(|_| EPubBook::zip_error("write", "toc navmap start")));

        let mut titles = 0;
        for &(_, ref fname, ref tp) in self.content.iter() {
            if let &EPubContentType::File(ref pb) = tp {
                if let Some(title) = find_title(&mut try!(File::open(pb).map_err(|_| {
                    Error::Io {
                        desc: "Content",
                        op: "open",
                        more: None,
                    }
                }))) {
                    let uuid = Uuid::new_v4();
                    titles += 1;

                    try!(writeln!(w, r#"    <navPoint id="{}" playOrder="{}">"#, uuid.hyphenated(), titles)
                        .map_err(|_| EPubBook::zip_error("write", "toc navmap point start")));
                    try!(writeln!(w, r#"      <navLabel>"#).map_err(|_| EPubBook::zip_error("write", "toc navmap label start")));
                    try!(writeln!(w, r#"        <text>{}</text>"#, title).map_err(|_| EPubBook::zip_error("write", "toc navmap label text")));
                    try!(writeln!(w, r#"      </navLabel>"#).map_err(|_| EPubBook::zip_error("write", "toc navmap label end")));
                    try!(writeln!(w, r#"      <content src="{}"/>"#, fname.display()).map_err(|_| EPubBook::zip_error("write", "toc navmap point content")));
                    try!(writeln!(w, r#"    </navPoint>"#).map_err(|_| EPubBook::zip_error("write", "toc navmap point end")));

                    if verbose {
                        let _ = writeln!(verb_out, r#"Found title "{}" for {}."#, title, fname.display());
                    }
                }
            }
        }

        try!(writeln!(w, r#"  </navMap>"#).map_err(|_| EPubBook::zip_error("write", "toc navmap end")));
        try!(writeln!(w, r#"</ncx>"#).map_err(|_| EPubBook::zip_error("write", "toc ncx end")));

        Ok(())
    }

    fn write_content<W: Write + Seek, V: Write>(&self, w: &mut ZipWriter<W>, verbose: bool, verb_out: &mut V) -> Result<(), Error> {
        for &(_, ref fname, ref tp) in self.cover.iter().chain(self.content.iter()).chain(self.non_content.iter()) {
            try!(w.start_file(fname.to_str().unwrap(), FileOptions::default()).map_err(|_| EPubBook::zip_error("create", "table of contents")));
            match *tp {
                EPubContentType::File(ref pb) => {
                    try!(io::copy(&mut try!(File::open(pb).map_err(|_| EPubBook::zip_error("open", "Content file"))), w)
                        .map_err(|_| EPubBook::zip_error("write", "Content data")));
                }
                EPubContentType::Network(ref u) => {
                    if verbose {
                        let _ = writeln!(verb_out, "Downloading {} to {}.", u, fname.display());
                    }
                    try!(download_to(w, &u));
                }
                EPubContentType::Raw(ref s) => try!(write_string_content(w, s)),
            }
        }

        Ok(())
    }
}
