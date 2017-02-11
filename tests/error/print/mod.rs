mod parse;
mod io;

use std::path::PathBuf;
use gen_epub_book::Error;


#[test]
fn file_not_found() {
    let mut out = Vec::new();
    Error::FileNotFound {
            who: "Content",
            path: PathBuf::from("file/that/does/not.exist"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "File file/that/does/not.exist for Content not found.\n".to_string());
}

#[test]
fn wrong_file_state() {
    let mut out = Vec::new();
    Error::WrongFileState {
            what: "actually a file",
            path: PathBuf::from("file/that/does/not.exist"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "File file/that/does/not.exist is not actually a file.\n".to_string());
}
