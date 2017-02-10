use gen_epub_book::ops::BookElement;

mod errors;
mod elements;


#[test]
fn unrecognised() {
    assert_eq!(BookElement::parse("Network-Content: https://nabijaczleweli.xyz/capitalism/extensive_list_of_rust_puns/index.html"),
               Ok(None));
    assert_eq!(BookElement::parse("For example: exhuberant capitalism combined with an aberration of the self"),
               Ok(None));
}

#[test]
fn not_description() {
    assert_eq!(BookElement::parse("# Simple thing, should work"), Ok(None));
    assert_eq!(BookElement::parse("Workers all over the world, unite!"), Ok(None));
}

#[test]
fn trimming() {
    let result = Ok(Some(BookElement::Name("nabijaczleweli".to_string())));
    assert_eq!(BookElement::parse("Name:nabijaczleweli"), result);
    assert_eq!(BookElement::parse("Name: nabijaczleweli"), result);
    assert_eq!(BookElement::parse("Name: nabijaczleweli "), result);
    assert_eq!(BookElement::parse("Name: nabijaczleweli\n"), result);
    assert_eq!(BookElement::parse("Name : nabijaczleweli"), result);
    assert_eq!(BookElement::parse("  Name : nabijaczleweli \n"), result);
}

#[test]
fn roundtrip() {
    assert_eq!(&BookElement::parse("Name: nabijaczleweli").unwrap().unwrap().to_string(),
               "Name: nabijaczleweli");
}
