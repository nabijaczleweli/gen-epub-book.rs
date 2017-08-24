//! Generate an ePub book from a simple plaintext descriptor
//!
//! # Library doc
//!
//! This library is used by `gen-epub-book` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! ```text
//! Options
//! |> parse_descriptor()
//! |> EPubBook::from_elements()
//! |> EPubBook::normalise_paths()
//! |> EPubBook::write_zip()
//! ```
//!
//! # Executable manpage
//!
//! Exit values and possible errors:
//!
//! ```text
//! 1 - I/O error
//! 2 - parsing error
//! 3 - file not found
//! 4 - file in wrong state
//! 5 - incorrect amount of elements
//! 6 - required element missing
//! ```
//!
//! ## SYNOPSIS
//!
//! [`gen-epub-book`](https://github.com/nabijaczleweli/gen-epub-book.rs) [OPTIONS] IN_FILE OUT_FILE
//!
//! ## DESCRIPTION
//!
//! Generate an ePub book from a simple plaintext descriptor.
//!
//! ## OPTIONS
//!
//! -v --verbose
//!
//! ```text
//! Print out more data.
//!
//! Default: false.
//! ```
//!
//! IN_FILE
//!
//! ```text
//! File to parse, must exist, must comply with the DESCRIPTOR FORMAT.
//!
//! Special case: '-' to read from stdin.
//! ```
//!
//! OUT_FILE
//!
//! ```text
//! File to write the book to, parent directory needn't exist.
//!
//! Special case: '-' to write to stdout.
//! ```
//!
//! -S --separator &lt;SEPARATOR&gt;
//!
//! ```text
//! Enable custom separator feature and set the separator.
//!
//! Default: ":".
//! ```
//!
//! -I --include [NAME=]PATH
//!
//! ```text
//! Add an additional directory in which to search for files. Order-dependent.
//!
//! `NAME` is an optional name under which the files will be segregated.
//! `PATH` is an existing directory.
//! ```
//!
//! ## DESCRIPTOR FORMAT
//!
//! The descriptor consists of multiple lines in the format *"Key: Value"*, unknown
//! keys are ignored, lines that don't match the format are ignored.
//!
//! Name
//!
//! ```text
//! Required: yes
//! Type: plaintext
//! Value: e-book's title
//! Amount: 1
//! ```
//!
//! Content
//!
//! ```text
//! Required: no
//! Type: file path
//! Value: relative path to (X)HTML chunk
//! Amount: any
//! Remarks: see ADDITIONAL CONTENT PROCESSING
//! ```
//!
//! String-Content
//!
//! ```text
//! Required: no
//! Type: (X)HTML
//! Value: (X)HTML string
//! Amount: any
//! ```
//!
//! Image-Content
//!
//! ```text
//! Required: no
//! Type: file path
//! Value: relative path to image to include in e-book
//! Amount: any
//! ```
//!
//! Network-Image-Content
//!
//! ```text
//! Required: no
//! Type: file URL
//! Value: URL of image to include in e-book
//! Amount: any
//! ```
//!
//! Cover
//!
//! ```text
//! Required: no
//! Type: file path
//! Value: relative path to image to use as e-book cover
//! Amount: 0-1
//! Remarks: exclusive with Network-Cover
//! ```
//!
//! Network-Cover
//!
//! ```text
//! Required: no
//! Type: file URL
//! Value: URL to image to use as e-book cover
//! Amount: 0-1
//! Remarks: exclusive with Cover
//! ```
//!
//! Author
//!
//! ```text
//! Required: yes
//! Type: plaintext string
//! Value: e-book's author
//! Amount: 1
//! ```
//!
//! Date
//!
//! ```text
//! Required: yes
//! Type: RFC3339-compliant date
//! Value: e-book's authoring/publishing date
//! Amount: 1
//! ```
//!
//! Language
//!
//! ```text
//! Required: yes
//! Type: BCP47-compliant language code
//! Value: language used in e-book
//! Amount: 1
//! ```
//!
//! ## ADDITIONAL CONTENT PROCESSING
//!
//! When adding content using the `Content` entry, the file will additinally be
//! searched for a comment specifying the its name in the TOC in this format:
//!
//! ```text
//! <!-- ePub title: "TOC_NAME" -->
//! ```
//!
//! Where `TOC_NAME` is a string not containing the *"* character.
//!
//! This will, on e-book readers, allow users to jump directly to the content
//! represented by the document containing this entry.
//!
//! Optional.


#[macro_use]
extern crate lazy_static;
extern crate mime_guess;
extern crate reqwest;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate clap;
extern crate uuid;
extern crate url;
extern crate zip;

mod error;
mod options;

pub mod ops;
pub mod util;

pub use error::Error;
pub use options::Options;
