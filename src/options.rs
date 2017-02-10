//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use gen_epub_book::Options;
//! let options = Options::parse();
//! println!("Assembling {} to {}", options.source_file.0, options.output_file.0);
//! ```


use clap::{AppSettings, Arg};
use std::path::PathBuf;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The descriptor file.
    pub source_file: (String, PathBuf),
    /// The root for relative source paths.
    pub relative_root: (String, PathBuf),
    /// The file to insert the assembled ePub to.
    pub output_file: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<SOURCE> 'File to assemble ePub from'").validator(Options::source_file_validator))
            .arg(Arg::from_usage("<TARGET> 'File to write'"))
            .get_matches();

        let source = matches.value_of("SOURCE").unwrap();
        let target = matches.value_of("TARGET").unwrap();
        Options {
            source_file: (source.to_string(), PathBuf::from(source)),
            relative_root: match source.rfind('/').or_else(|| source.rfind("\\")) {
                Some(s) => (source[..s + 1].to_string(), PathBuf::from(&source[..s])),
                None => ("".to_string(), PathBuf::from(".")),
            },
            output_file: (target.to_string(), PathBuf::from(target)),
        }
    }

    fn source_file_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map_err(|_| format!("Source file \"{}\" not found", s)).and_then(|f| if f.is_file() {
            Ok(())
        } else {
            Err(format!("Source file \"{}\" not actualy a file", s))
        })
    }
}
