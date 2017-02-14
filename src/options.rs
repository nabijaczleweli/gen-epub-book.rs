//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use gen_epub_book::Options;
//! let options = Options::parse();
//! println!("{}sing verbose output", if options.verbose {"U"} else {"Not u"});
//! ```


use clap::{AppSettings, Arg};
use std::path::PathBuf;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The descriptor file, or `None` for stdin.
    pub source_file: Option<(String, PathBuf)>,
    /// The root for relative source paths.
    pub relative_root: (String, PathBuf),
    /// The file to insert the assembled ePub to, or `None` for stdout.
    pub output_file: Option<(String, PathBuf)>,
    /// Whether to print more information. Default: false
    pub verbose: bool,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<SOURCE> 'File to assemble ePub from'").validator(Options::source_file_validator))
            .arg(Arg::from_usage("<TARGET> 'File to write'"))
            .arg(Arg::from_usage("-v --verbose 'Print more information'"))
            .get_matches();

        let source = Options::optional_fname_arg(matches.value_of("SOURCE").unwrap());
        let target = Options::optional_fname_arg(matches.value_of("TARGET").unwrap());
        Options {
            source_file: source.map(|s| (s.to_string(), PathBuf::from(s))),
            relative_root: match source.and_then(|src| src.rfind('/').or_else(|| src.rfind('\\'))) {
                Some(s) => (source.unwrap()[..s + 1].to_string(), PathBuf::from(&source.unwrap()[..s])),
                None => ("".to_string(), PathBuf::from(".")),
            },
            output_file: target.map(|tgt| (tgt.to_string(), PathBuf::from(tgt))),
            verbose: matches.is_present("verbose"),
        }
    }

    fn source_file_validator(s: String) -> Result<(), String> {
        if s == "-" {
            Ok(())
        } else {
            fs::canonicalize(&s).map_err(|_| format!("Source file \"{}\" not found", s)).and_then(|f| if f.is_file() {
                Ok(())
            } else {
                Err(format!("Source file \"{}\" not actualy a file", s))
            })
        }
    }

    fn optional_fname_arg(s: &str) -> Option<&str> {
        if s == "-" { None } else { Some(s) }
    }
}
