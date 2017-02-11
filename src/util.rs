//! Module containing various utility functions.


use std::path::Path;


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
/// # use gen_epub_book::util::xhtml_id;
/// assert_eq!(xhtml_id("./abolish/the/../burgeoisie.html"), "abolish-the-burgeoisie".to_string());
/// ```
pub fn xhtml_id<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().with_extension("").to_string_lossy().replace('\u{FFFD}', "").replace('\\', "/").replace("../", "").replace("./", "").replace('/', "-")
}
