gen-epub-book.rs(1) -- Generate an ePub book from a simple plaintext descriptor
===============================================================================

## SYNOPSIS

`gen-epub-book` IN_FILE OUT_FILE [OPTIONS]

## DESCRIPTION

Generate an ePub book from a simple plaintext descriptor.

Exit values and possible errors:

    1 - I/O error
    2 - parsing error
    3 - file not found
    4 - file in wrong state
    5 - incorrect amount of elements
    6 - required element missing

## OPTIONS

  -v --verbose

    Print out more data.

    Default: false.

  IN_FILE

    File to parse, must exist, must comply with the DESCRIPTOR FORMAT.

    Special case: '-' to read from stdin.

  OUT_FILE

    File to write the book to, parent directory needn't exist.

    Special case: '-' to write to stdout.

  -S --separator <SEPARATOR>

    Enable custom separator feature and set the separator.

    Default: ":".

  -I --include [NAME=]PATH

    Add an additional directory in which to search for files. Order-dependent.

    NAME is an optional name under which the files will be segregated.
    PATH is an existing directory.

## DESCRIPTOR FORMAT

The descriptor consists of multiple lines in the format *"Key: Value"*, unknown
keys are ignored, lines that don't match the format are ignored.

  Name

    Required: yes
    Type: plaintext
    Value: e-book's title
    Amount: 1

  Content

    Required: no
    Type: file path
    Value: relative path to (X)HTML chunk
    Amount: any
    Remarks: see ADDITIONAL CONTENT PROCESSING

  String-Content

    Required: no
    Type: (X)HTML
    Value: (X)HTML string
    Amount: any

  Image-Content

    Required: no
    Type: file path
    Value: relative path to image to include in e-book
    Amount: any

  Network-Image-Content

    Required: no
    Type: file URL
    Value: URL of image to include in e-book
    Amount: any

  Cover

    Required: no
    Type: file path
    Value: relative path to image to use as e-book cover
    Amount: 0-1
    Remarks: exclusive with Network-Cover

  Network-Cover

    Required: no
    Type: file URL
    Value: URL to image to use as e-book cover
    Amount: 0-1
    Remarks: exclusive with Cover

  Include

    Required: no
    Type: file path
    Value: auxilliary file to include in e-book
    Amount: any

  Network-Include

    Required: no
    Type: file URL
    Value: URL of auxilliary file to include in e-book
    Amount: any

  Author

    Required: yes
    Type: plaintext string
    Value: e-book's author
    Amount: 1

  Date

    Required: yes
    Type: RFC3339-compliant date
    Value: e-book's authoring/publishing date
    Amount: 1

  Language

    Required: yes
    Type: BCP47-compliant language code
    Value: language used in e-book
    Amount: 1

## ADDITIONAL CONTENT PROCESSING

When adding content using the `Content` entry, the file will additinally be
searched for a comment specifying the its name in the TOC in this format:

    <!-- ePub title: "TOC_NAME" -->

Where `TOC_NAME` is a string not containing the *"* character.

This will, on e-book readers, allow users to jump directly to the content
represented by the document containing this entry.

Optional.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/gen-epub-book.rs/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/gen-epub-book.rs>&gt;<br />
&lt;<https://nabijaczleweli.xyz/content/gen-epub-book>&gt;
