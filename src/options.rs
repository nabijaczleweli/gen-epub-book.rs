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


use self::super::ops::IncludeDirectory;
use clap::{AppSettings, Arg};
use std::path::PathBuf;
use std::iter;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The descriptor file, or `None` for stdin.
    pub source_file: Option<(String, PathBuf)>,
    /// The roots for relative source paths, or, so called
    /// [`-I`nclude dirs](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-include-dirs).
    ///
    /// Default:
    ///
    ///   * `["."]` if `source_file == None`, or
    ///   * `[`directory containing `source_file``]` otherwise.
    pub include_directories: Vec<IncludeDirectory>,
    /// The file to insert the assembled ePub to, or `None` for stdout.
    pub output_file: Option<(String, PathBuf)>,
    /// Whether to print more information.
    ///
    /// Default: false
    pub verbose: bool,
    /// Chosen [separator](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-custom-separator).
    ///
    /// Default: `":"`
    pub separator: String,
    /// Whether to (attempt to) parse
    /// [more date formats](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-free-date-format)
    /// than just RFC3339.
    ///
    /// Default: false
    pub free_date: bool,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<SOURCE> 'File to assemble ePub from'").validator(Options::source_file_validator))
            .arg(Arg::from_usage("<TARGET> 'File to write'"))
            .arg(Arg::from_usage("-v --verbose 'Print more information'"))
            .arg(Arg::from_usage("-D --free-date 'Parse more datetime formats'"))
            .arg(Arg::from_usage("-S --separator [SEPARATOR] 'Custom separator'").default_value(":").validator(Options::separator_validator).required(false))
            .arg(Arg::from_usage("-I --include [INC_DIR]... 'Additional include directory. Format: [name=]path'")
                .validator(Options::include_dir_validator)
                .required(false))
            .get_matches();

        let source = Options::optional_fname_arg(matches.value_of("SOURCE").unwrap());
        let target = Options::optional_fname_arg(matches.value_of("TARGET").unwrap());
        let source_root = match source.and_then(|src| src.rfind('/').or_else(|| src.rfind('\\'))) {
            Some(s) => IncludeDirectory::Unnamed { dir: (source.unwrap()[..s + 1].to_string(), PathBuf::from(&source.unwrap()[..s])) },
            None => IncludeDirectory::Unnamed { dir: ("".to_string(), PathBuf::from(".")) },
        };
        Options {
            source_file: source.map(|s| (s.to_string(), PathBuf::from(s))),
            include_directories: iter::once(source_root)
                .chain(matches.values_of("include").into_iter().flat_map(|v| v.map(str::parse).map(Result::unwrap)))
                .collect(),
            output_file: target.map(|tgt| (tgt.to_string(), PathBuf::from(tgt))),
            verbose: matches.is_present("verbose"),
            separator: matches.value_of("separator").unwrap_or(":").to_string(),
            free_date: matches.is_present("free-date"),
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

    fn separator_validator(s: String) -> Result<(), String> {
        if s.is_empty() {
            Err("Separator empty".to_string())
        } else {
            Ok(())
        }
    }

    fn include_dir_validator(s: String) -> Result<(), String> {
        s.parse::<IncludeDirectory>().map(|_| ()).map_err(|e| {
            let mut out = Vec::new();
            e.print_error(&mut out);
            String::from_utf8(out).unwrap()
        })
    }

    fn optional_fname_arg(s: &str) -> Option<&str> {
        if s == "-" { None } else { Some(s) }
    }
}
