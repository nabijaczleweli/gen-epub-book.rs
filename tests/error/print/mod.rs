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

#[test]
fn wrong_element_amount() {
    let mut out = Vec::new();
    Error::WrongElementAmount {
            element: "Name",
            actual: 2,
            relation: "exactly",
            bound: 1,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "Wrong amount of Name elements: 2, must be exactly 1.\n".to_string());
}

#[test]
fn required_element_missing() {
    let mut out = Vec::new();
    Error::RequiredElementMissing("Name").print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "Required element Name not specified.\n".to_string());
}
