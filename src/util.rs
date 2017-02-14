//! Module containing various utility functions.


use std::path::{PathBuf, Path};
use std::io::{self, Write};
use self::super::Error;
use url::Url;
use reqwest;


/// Contents of the the container file
///
/// Points at `/content.opf` for root file.
///
/// Container file resides in `META-INF/container.xml`.
pub static CONTAINER: &'static str = include_str!("../assets/container.xml");

/// Contents of the the mimetype specifier.
///
/// Mimetype specifier resides in `/mimetype`.
pub static MIME_TYPE: &'static str = "application/epub+zip";

/// Constant header for the root file/content table.
///
/// Mimetype specifier resides in `/content.opf`.
pub static CONTENT_TABLE_HEADER: &'static str = include_str!("../assets/content.opf.header");


/// Uppercase the first character of the supplied string.
///
/// Based on http://stackoverflow.com/a/38406885/2851815
///
/// # Examples
///
/// ```
/// # use gen_epub_book::util::uppercase_first;
/// assert_eq!(uppercase_first("abolish"), "Abolish".to_string());
/// ```
pub fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Get the (X)HTML ID from a path.
///
/// Replaces slashes with `-`s and removes all `../`s, `./`s and the extension.
///
/// # Examples
///
/// ```
/// # use gen_epub_book::util::xhtml_path_id;
/// assert_eq!(&xhtml_path_id("./abolish/the/../burgeoisie.html"), "abolish-the-burgeoisie");
/// ```
pub fn xhtml_path_id<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().with_extension("").to_string_lossy().replace('\u{FFFD}', "").replace('\\', "/").replace("../", "").replace("./", "").replace('/', "-")
}

/// Get filename to use for file specified by path.
///
/// # Examples
///
/// ```
/// # use gen_epub_book::util::book_filename;
/// # use std::path::Path;
/// assert_eq!(&book_filename("./abolish/the/../burgeoisie.html"), Path::new("abolish-the-burgeoisie.html"));
/// ```
pub fn book_filename<P: AsRef<Path>>(p: P) -> PathBuf {
    let mut f = PathBuf::from(xhtml_path_id(p.as_ref()));
    if let Some(e) = p.as_ref().extension() {
        f.set_extension(e);
    }
    f
}

/// Get the (X)HTML ID from a URL.
///
/// # Examples
///
/// ```
/// # extern crate gen_epub_book;
/// # extern crate url;
/// # fn main() {
/// # use gen_epub_book::util::xhtml_url_id;
/// # use url::Url;
/// assert_eq!(xhtml_url_id(
///   &Url::parse("https://upload.wikimedia.org/2000px-Recycle_symbol_Taiwan.svg.png").unwrap()),
///   "2000px-Recycle_symbol_Taiwan");
/// # }
/// ```
pub fn xhtml_url_id(url: &Url) -> &str {
    let fname = url.path_segments().unwrap().last().unwrap();
    &fname[0..fname.find('.').unwrap_or(fname.len() - 1)]
}

/// Write the string content in an acceptable form.
///
/// # Examples
///
/// ```
/// # use gen_epub_book::util::write_string_content;
/// let mut buf = vec![];
/// assert_eq!(write_string_content(&mut buf, "<i>String content</i>"), Ok(()));
/// assert_eq!(&buf.iter().map(|&i| i as char).collect::<String>(),
/// r#"<html xmlns="http://www.w3.org/1999/xhtml">
///   <head></head>
///   <body>
///     <i>String content</i>
///   </body>
/// </html>
/// "#);
/// ```
pub fn write_string_content<W: Write>(to: &mut W, ctnt: &str) -> Result<(), Error> {
    fn e(more: &'static str) -> Error {
        Error::Io {
            desc: "string content",
            op: "write",
            more: Some(more),
        }
    }

    try!(writeln!(to, r#"<html xmlns="http://www.w3.org/1999/xhtml">"#).map_err(|_| e("string content html start")));
    try!(writeln!(to, r#"  <head></head>"#).map_err(|_| e("string content head start end")));
    try!(writeln!(to, r#"  <body>"#).map_err(|_| e("string content body start")));
    try!(writeln!(to, r#"    {}"#, ctnt).map_err(|_| e("string content string")));
    try!(writeln!(to, r#"  </body>"#).map_err(|_| e("string content body end")));
    try!(writeln!(to, r#"</html>"#).map_err(|_| e("string content html end")));

    Ok(())
}

/// Download the contents of the specified URL to the specified output stream.
///
/// # Examples
///
/// ```
/// # extern crate gen_epub_book;
/// # extern crate url;
/// # fn main() {
/// # use self::gen_epub_book::util::download_to;
/// # use self::url::Url;
/// let mut buf = vec![];
/// assert_eq!(download_to(&mut buf,
///                        &Url::parse("https://www.uuidgenerator.net/api/version4").unwrap()),
///            Ok(()));
/// assert_eq!(buf.len(), "90772bc3-c7fd-4d3a-b88b-57d3122d3712\r\n".len());
/// # }
/// ```
pub fn download_to<W: Write>(w: &mut W, what: &Url) -> Result<(), Error> {
    let mut resp = try!(reqwest::get(what.as_str()).map_err(|_| {
        Error::Io {
            desc: "network content",
            op: "request",
            more: None,
        }
    }));

    if !resp.status().is_success() {
        Err(Error::Io {
            desc: "network content",
            op: "inspect",
            more: resp.status().canonical_reason(),
        })
    } else {
        try!(io::copy(&mut resp, w).map_err(|_| {
            Error::Io {
                desc: "network content",
                op: "read",
                more: None,
            }
        }));
        Ok(())
    }
}
