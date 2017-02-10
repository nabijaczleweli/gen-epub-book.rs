use gen_epub_book::Error;


#[test]
fn io() {
    assert_eq!(Error::Io {
                       desc: "",
                       op: "",
                       more: None,
                   }
                   .exit_value(),
               1);
    assert_eq!(Error::Io {
                       desc: "",
                       op: "",
                       more: Some(""),
                   }
                   .exit_value(),
               1);
}

#[test]
fn parse() {
    assert_eq!(Error::Parse { tp: "", wher: "", more: None }.exit_value(), 2);
    assert_eq!(Error::Parse { tp: "", wher: "", more: Some("") }.exit_value(), 2);
}
