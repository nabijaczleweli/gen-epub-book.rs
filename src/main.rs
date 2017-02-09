extern crate gen_epub_book;

use std::io::stderr;
use std::process::exit;


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

fn result_main() -> Result<(), gen_epub_book::Error> {
    let opts = gen_epub_book::Options::parse();

    println!("{:#?}", opts);

    Ok(())
}
