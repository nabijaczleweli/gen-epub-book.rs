use gen_epub_book::Error;


#[test]
fn io() {
    assert_eq!(Error::Io { desc: "", op: "", more: None }.exit_value(), 1);
    assert_eq!(Error::Io { desc: "", op: "", more: Some("") }.exit_value(), 1);
}
