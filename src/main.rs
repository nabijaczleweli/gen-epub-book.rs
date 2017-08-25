extern crate gen_epub_book;

use std::process::exit;
use std::fs::{self, File};
use std::io::{Write, Cursor, stdin, stderr, stdout};
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

    let descriptors = try!(if let Some(infile) = opts.source_file.as_ref() {
        ops::parse_descriptor("input file",
                              &mut try!(File::open(&infile.1).map_err(|_| {
            Error::Io {
                desc: "input file",
                op: "open",
                more: None,
            }
        })),
                              &opts.separator)
    } else {
        ops::parse_descriptor("stdin", &mut stdin(), &opts.separator)
    });

    if opts.verbose {
        let _ = writeln!(stderr(),
                         "Loaded descriptor {}{} with {} entries.",
                         if opts.source_file.is_some() {
                             "file "
                         } else {
                             ""
                         },
                         if let Some(infile) = opts.source_file.as_ref() {
                             &infile.0
                         } else {
                             ""
                         },
                         descriptors.len());
    }

    let mut book = try!(ops::EPubBook::from_elements(descriptors));
    try!(book.normalise_paths(&opts.include_directories, opts.verbose, &mut stderr()));

    if let Some(outfile) = opts.output_file.as_ref() {
        if let Some(p) = outfile.1.parent() {
            if !p.as_os_str().is_empty() && !p.exists() && fs::create_dir_all(p).is_ok() && opts.verbose {
                let _ = writeln!(stderr(),
                                 "Created directory {}.",
                                 outfile.0[..outfile.0.rfind('/').or_else(|| outfile.0.rfind('\\')).map(|i| i + 1).unwrap_or(0)].to_string());
            }
        }
    }

    if let Some(outfile) = opts.output_file.as_ref() {
        let mut outf = try!(File::create(&outfile.1).map_err(|_| {
            Error::Io {
                desc: "output file",
                op: "create",
                more: None,
            }
        }));
        try!(book.write_zip(&mut outf, opts.verbose, &mut stderr()));
    } else {
        let mut buf = Cursor::new(vec![]);
        try!(book.write_zip(&mut buf, opts.verbose, &mut stderr()));
        try!(stdout().write_all(buf.get_ref()).map_err(|_| {
            Error::Io {
                desc: "stdout",
                op: "write",
                more: Some("final ePub"),
            }
        }));
    }

    Ok(())
}
