use gen_epub_book::ops::find_title;
use std::fs::File;


#[test]
fn containing() {
    assert_eq!(find_title(&mut File::open("examples/simple/ctnt.html").unwrap()),
               Some("Chapter 1, Where everything's still going mostly right".to_string()));
    assert_eq!(find_title(&mut File::open("examples/relative_path_fuckery/six_minutes.html").unwrap()),
               Some("Six Minutes".to_string()));
    assert_eq!(find_title(&mut File::open("examples/relative_path_fuckery/relative/green_ass_dog.html").unwrap()),
               Some("Cucumber Dog".to_string()));
    assert_eq!(find_title(&mut File::open("examples/relative_path_fuckery/relative/path/dead_santa.html").unwrap()),
               Some("No gifts this year".to_string()));
    assert_eq!(find_title(&mut File::open("src/ops/mod.rs").unwrap()), Some("TOC_NAME".to_string()));
}

#[test]
fn non_containing() {
    assert_eq!(find_title(&mut File::open("examples/relative_path_fuckery/relative/path/not_dead_yet.png").unwrap()),
               None);
    assert_eq!(find_title(&mut File::open("examples/relative_path_fuckery/relative_path_fuckery.epupp").unwrap()),
               None);
    assert_eq!(find_title(&mut &br#"<!-- ePub title: "TOC_NAME -->"#[..]), None);
    assert_eq!(find_title(&mut &br#"<!-- ePub title: "" -->"#[..]), None);
}
