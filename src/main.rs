extern crate gen_epub_book;

use std::process::exit;
use std::fs::{self, File};
use std::io::{Write, stderr};
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

    let descriptors = try!(ops::parse_descriptor("input file",
                                                 &mut try!(File::open(&opts.source_file.1).map_err(|_| {
        Error::Io {
            desc: "input file",
            op: "open",
            more: None,
        }
    }))));

    if opts.verbose {
        let _ = writeln!(stderr(), "Loaded descriptor file {} with {} entries.", opts.source_file.0, descriptors.len());
    }

    let mut book = try!(ops::EPubBook::from_elements(descriptors));
    try!(book.normalise_paths(&opts.relative_root, opts.verbose, &mut stderr()));

    if let Some(p) = opts.output_file.1.parent() {
        if !p.exists() && fs::create_dir_all(p).is_ok() && opts.verbose {
            let _ = writeln!(stderr(),
                             "Created directory {}.",
                             opts.output_file.0[..opts.output_file.0.rfind('/').or_else(|| opts.output_file.0.rfind('\\')).unwrap() + 1].to_string());
        }
    }
    let mut outf = try!(File::create(&opts.output_file.1).map_err(|_| {
        Error::Io {
            desc: "output file",
            op: "create",
            more: None,
        }
    }));
    try!(book.write_zip(&mut outf, opts.verbose, &mut stderr()));

    Ok(())
}
