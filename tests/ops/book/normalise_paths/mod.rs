mod no_verbose;
mod verbose;

use std::fs;
use std::env;
use std::path::PathBuf;


fn make_test_folder(f: &str) -> PathBuf {
    let mut td = env::temp_dir();
    let _ = fs::create_dir(&td);
    td.push("gen-epub-book.rs-test");
    let _ = fs::create_dir(&td);
    td.push(format!("ops-book-normalise-paths-{}", f));
    let _ = fs::create_dir(&td);
    td
}
