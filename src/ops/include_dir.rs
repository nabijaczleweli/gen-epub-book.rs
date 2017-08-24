use self::super::super::util::{xhtml_path_id, book_filename};
use std::path::{PathBuf, Path};
use self::super::super::Error;
use std::str::FromStr;
use std::fmt;
use std::fs;


/// Representation of an
/// [`-I`nclude directory](https://nabijaczleweli.xyz/content/gen-epub-book/programmer.html#features-include-dirs).
///
/// Textually,
/// *unnamed* `-I`nclude directories take the form of `"path"`, and
/// *named* `-I`nclude directories take the form of `"name=path"`.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum IncludeDirectory {
    /// An *unnamed* include directory, acting transparently
    Unnamed {
        /// Directory path
        dir: (String, PathBuf),
    },
    /// A *named* include directory, saved with prefixes.
    Named {
        /// `-I`nclude directory name.
        name: String,
        /// Directory path.
        dir: (String, PathBuf),
    },
}

impl IncludeDirectory {
    /// Get the name of the include directory
    ///
    /// (Also known as `self.dir.0`, but it's a convenience function, because :enums:.)
    ///
    /// # Examples
    ///
    /// ```
    /// # use gen_epub_book::ops::IncludeDirectory;
    /// # use std::path::PathBuf;
    /// assert_eq!(
    ///     IncludeDirectory::Unnamed {
    ///         dir: ("cur-dir".to_string(), PathBuf::from(".")),
    ///     }.directory_name(),
    ///     "cur-dir");
    /// assert_eq!(
    ///     IncludeDirectory::Named {
    ///         name: "dot".to_string(),
    ///         dir: ("named-cur-dir".to_string(), PathBuf::from(".")),
    ///     }.directory_name(),
    ///     "named-cur-dir");
    /// ```
    pub fn directory_name(&self) -> &str {
        match *self {
            IncludeDirectory::Named { ref dir, .. } => &dir.0,
            IncludeDirectory::Unnamed { ref dir } => &dir.0,
        }
    }

    /// Get packed filename for file specified by path.
    ///
    /// Basically optionally prefixes [`util::book_filename()`](../util/fn.book_filename.html).
    ///
    /// Path separator, if any, is always `'/'`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gen_epub_book::ops::IncludeDirectory;
    /// # use gen_epub_book::util::book_filename;
    /// # use std::path::{PathBuf, Path};
    /// let fname = Path::new("content/ch01.html");
    /// assert_eq!(
    ///     IncludeDirectory::Unnamed {
    ///         dir: ("cur-dir".to_string(), PathBuf::from(".")),
    ///     }.packed_name(&fname),
    ///     book_filename(&fname));
    /// assert_eq!(
    ///     IncludeDirectory::Named {
    ///         name: "dot".to_string(),
    ///         dir: ("named-cur-dir".to_string(), PathBuf::from(".")),
    ///     }.packed_name(&fname).display().to_string(),
    ///     format!("dot/{}", book_filename(&fname).display()));
    /// ```
    pub fn packed_name<P: AsRef<Path>>(&self, f: P) -> PathBuf {
        match *self {
            // Okay so here we can't just do Path::new(name).join(book_filename(f)) because that'll give us backslashes on Windows
            IncludeDirectory::Named { ref name, .. } => Path::new(name).join(book_filename(f)).to_str().unwrap().replace('\\', "/").into(),
            IncludeDirectory::Unnamed { .. } => book_filename(f),
        }
    }

    /// Get the (X)HTML ID from a path.
    ///
    /// Basically optionally prefixes [`util::xhtml_path_id()`](../util/fn.xhtml_path_id.html).
    ///
    /// # Examples
    ///
    /// ```
    /// # use gen_epub_book::ops::IncludeDirectory;
    /// # use gen_epub_book::util::xhtml_path_id;
    /// # use std::path::{PathBuf, Path};
    /// let fname = Path::new("content/ch01.html");
    /// assert_eq!(
    ///     IncludeDirectory::Unnamed {
    ///         dir: ("cur-dir".to_string(), PathBuf::from(".")),
    ///     }.packed_id(&fname),
    ///     xhtml_path_id(&fname));
    /// assert_eq!(
    ///     IncludeDirectory::Named {
    ///         name: "dot".to_string(),
    ///         dir: ("named-cur-dir".to_string(), PathBuf::from(".")),
    ///     }.packed_id(&fname),
    ///     format!("dot--{}", xhtml_path_id(&fname)));
    /// ```
    pub fn packed_id(&self, f: &Path) -> String {
        match *self {
            IncludeDirectory::Named { ref name, .. } => format!("{}--{}", name, xhtml_path_id(f)),
            IncludeDirectory::Unnamed { .. } => xhtml_path_id(f),
        }
    }

    /// Resolve the path of the specified file in this include directory, or `None` if nonexistant or isn't a file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gen_epub_book::ops::IncludeDirectory;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::path::Path;
    /// # let special_book = temp_dir().join("gen-epub-book.rs-doctest").join("ops-include-dir-resolve-0");
    /// # fs::create_dir_all(special_book.join("rendered").join("output")).unwrap();
    /// # fs::create_dir_all(special_book.join("previews").join("generated").join("out")).unwrap();
    /// # fs::create_dir_all(special_book.join("gep").join("special")).unwrap();
    /// # File::create(special_book.join("rendered").join("output").join("ending.html")).unwrap();
    /// # File::create(special_book.join("previews").join("generated").join("out").join("main.html")).unwrap();
    /// # File::create(special_book.join("gep").join("special").join("intro.html")).unwrap();
    /// let default_dir = special_book.join("gep").join("special");
    /// let previews_dir = special_book.join("previews").join("generated").join("out");
    /// let rendered_dir = special_book.join("rendered").join("output");
    /// let default = IncludeDirectory::Unnamed {
    ///     dir: ("".to_string(), default_dir.clone()),
    /// };
    /// let previews = IncludeDirectory::Named {
    ///     name: "previews".to_string(),
    ///     dir: ("../../previews/generated/out".to_string(), previews_dir.clone()),
    /// };
    /// let rendered = IncludeDirectory::Unnamed {
    ///     dir: ("../../rendered/output".to_string(), rendered_dir.clone()),
    /// };
    ///
    /// assert_eq!(default.resolve(Path::new("intro.html")), Some(default_dir.join("intro.html")));
    /// assert_eq!(previews.resolve(Path::new("main.html")), Some(previews_dir.join("main.html")));
    /// assert_eq!(rendered.resolve(Path::new("ending.html")),
    ///            Some(rendered_dir.join("ending.html")));
    /// assert_eq!(default.resolve(Path::new("cover.png")), None);
    /// assert_eq!(default.resolve(Path::new("../special")), None);
    /// ```
    pub fn resolve<P: AsRef<Path>>(&self, relpath: P) -> Option<PathBuf> {
        let abspath = match *self {
                IncludeDirectory::Named { name: _, ref dir } => dir,
                IncludeDirectory::Unnamed { ref dir } => dir,
            }
            .1
            .join(relpath);

        if abspath.exists() && abspath.is_file() {
            Some(abspath)
        } else {
            None
        }
    }
}

impl FromStr for IncludeDirectory {
    type Err = Error;

    fn from_str(s: &str) -> Result<IncludeDirectory, Error> {
        fn resolve_dir(from: &str) -> Result<(String, PathBuf), Error> {
            Ok((from.to_string(),
                try!(fs::canonicalize(from)
                    .map_err(|_| {
                    Error::Parse {
                        tp: "directory",
                        wher: "include directory",
                        more: Some("not found"),
                    }
                })
                    .and_then(|f| if !f.is_file() {
                        Ok(f)
                    } else {
                        Err(Error::WrongFileState {
                            what: "a directory",
                            path: PathBuf::from(from),
                        })
                    }))))
        }


        Ok(if let Some(idx) = s.find('=') {
            IncludeDirectory::Named {
                name: s[0..idx].to_string(),
                dir: try!(resolve_dir(&s[idx + 1..])),
            }
        } else {
            IncludeDirectory::Unnamed { dir: try!(resolve_dir(s)) }
        })
    }
}

impl fmt::Display for IncludeDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IncludeDirectory::Named { ref name, ref dir } => write!(f, "{}={}", name, dir.0),
            IncludeDirectory::Unnamed { ref dir } => write!(f, "{}", dir.0),
        }
    }
}
