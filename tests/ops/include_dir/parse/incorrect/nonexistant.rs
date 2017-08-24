use self::super::super::make_test_folder;
use gen_epub_book::ops::IncludeDirectory;
use gen_epub_book::Error;


#[test]
fn unnamed() {
    let tf = make_test_folder("incorrect-nonexistant-unnamed").join("dir");
    assert_eq!(tf.display().to_string().parse::<IncludeDirectory>(),
               Err(Error::Parse {
                   tp: "directory",
                   wher: "include directory",
                   more: Some("not found"),
               }));
}

#[test]
fn named() {
    let tf = make_test_folder("incorrect-nonexistant-named").join("dir");
    assert_eq!(format!("incorrect-nonexistant-named={}", tf.display()).parse::<IncludeDirectory>(),
               Err(Error::Parse {
                   tp: "directory",
                   wher: "include directory",
                   more: Some("not found"),
               }));
}
