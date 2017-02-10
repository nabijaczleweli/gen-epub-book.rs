use gen_epub_book::Error;


#[test]
fn normal_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "read",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "Reading input file failed: stream ended.\n".to_string());
}

#[test]
fn normal_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "output file",
            op: "create",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "Creating output file failed: stream ended.\n".to_string());
}

#[test]
fn single_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "C",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "Cing input file failed: stream ended.\n".to_string());
}

#[test]
fn single_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "e",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "ing input file failed: stream ended.\n".to_string());
}

#[test]
fn empty() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file",
            op: "",
            more: Some("stream ended"),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "ing input file failed: stream ended.\n".to_string());
}
