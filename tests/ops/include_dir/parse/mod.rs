use std::path::PathBuf;
use std::env;
use std::fs;


mod correct;
mod incorrect;


fn make_test_folder(f: &str) -> PathBuf {
    let mut td = env::temp_dir();
    let _ = fs::create_dir(&td);
    td.push("gen-epub-book.rs-test");
    let _ = fs::create_dir(&td);
    td.push(format!("ops-include-dir-parse-{}", f));
    let _ = fs::create_dir(&td);
    td
}
