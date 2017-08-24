use self::super::super::make_test_folder;
use gen_epub_book::ops::IncludeDirectory;
use gen_epub_book::Error;
use std::fs::File;


#[test]
fn unnamed() {
    let tf = make_test_folder("incorrect-invalid-unnamed").join("file");
    File::create(&tf).unwrap();
    assert_eq!(tf.display().to_string().parse::<IncludeDirectory>(),
               Err(Error::WrongFileState {
                   what: "a directory",
                   path: tf,
               }));
}

#[test]
fn named() {
    let tf = make_test_folder("incorrect-invalid-named").join("file");
    File::create(&tf).unwrap();
    assert_eq!(format!("incorrect-invalid-named={}", tf.display()).parse::<IncludeDirectory>(),
               Err(Error::WrongFileState {
                   what: "a directory",
                   path: tf,
               }));
}
