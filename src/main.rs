extern crate gen_epub_book;

use std::fs::File;
use std::io::stderr;
use std::process::exit;
use self::gen_epub_book::{ops, Options, Error};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), Error> {
    let opts = Options::parse();
    println!("{:#?}", opts);

    for elem in try!(ops::parse_descriptor("input file",
                                           &mut try!(File::open(&opts.source_file.1).map_err(|_| {
        Error::Io {
            desc: "input file",
            op: "open",
            more: None,
        }
    })))) {
        println!("{}", elem);
    }

    Ok(())
}
