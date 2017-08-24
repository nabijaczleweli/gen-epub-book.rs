use gen_epub_book::ops::IncludeDirectory;
use self::super::make_test_folder;
use std::fs;


#[test]
fn unnamed() {
    let td = make_test_folder("correct-unnamed");
    assert_eq!(td.display().to_string().parse(),
               Ok(IncludeDirectory::Unnamed { dir: (td.display().to_string(), fs::canonicalize(td).unwrap()) }));
}

#[test]
fn named() {
    let td = make_test_folder("correct-named");
    assert_eq!(format!("correct-named={}", td.display()).parse(),
               Ok(IncludeDirectory::Named {
                   name: "correct-named".to_string(),
                   dir: (td.display().to_string(), fs::canonicalize(td).unwrap()),
               }));
}
