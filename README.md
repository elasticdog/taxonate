# taxonate

[![Rust (stable)](<https://github.com/elasticdog/taxonate/workflows/Rust%20(stable)/badge.svg>)](https://github.com/elasticdog/taxonate/actions?query=workflow%3A%22Rust+%28stable%29%22+branch%3Amaster)

A command line tool to identify and filter plain text files based on their
[_programming_ language]. This can be useful for automated formatting and
linting of code as part of continuous integration.

Language identification is established by first checking the file for a
[shebang] line with a known interpreter; if none is found, the file name itself
is checked against known glob patterns. This two-step process is more accurate
than a naive search for file [extensions], as it will properly classify
executable scripts.

[_programming_ language]: https://en.wikipedia.org/wiki/Programming_language
[shebang]: https://en.wikipedia.org/wiki/Shebang_(Unix)
[extensions]: https://en.wikipedia.org/wiki/Filename_extension

## Usage

    taxonate [FLAGS] [OPTIONS] [PATH]...

Run `taxonate --help` for detailed information on all features.

### Identification

In its most simple form, point `taxonate` at a file to identify its language:

    $ taxonate README.md
    README.md: Markdown

File names can also be read from STDIN, if you specify a dash (`-`):

    $ find . -name "main*" | taxonate -
    ./target/doc/main.js: JavaScript
    ./src/bin/taxonate/main.rs: Rust

Instead of pointing to individual files, you can point to a directory to
recursively identify all files within it (respecting [`.gitignore`] patterns):

    $ taxonate src/
    src/bin/taxonate/main.rs: Rust
    src/bin/taxonate/app.rs: Rust
    src/lib.rs: Rust
    src/languages.rs: Rust
    src/config.rs: Rust

> _NOTE:_ If no path is provided, `taxonate` will default to recursively
> identifying files within the current directory.

[`.gitignore`]: https://git-scm.com/docs/gitignore

### Filtering

To filter the output so it only displays files identified as a specific
language, use the `--language` option:

    $ taxonate --language rust
    ./src/bin/taxonate/main.rs: Rust
    ./src/bin/taxonate/app.rs: Rust
    ./src/lib.rs: Rust
    ./src/languages.rs: Rust
    ./src/config.rs: Rust

To display just the file names without the identified language (e.g. if you want
to pipe the output elsewhere), add the `--filename-only` flag:

    $ taxonate --language rust --filename-only
    ./src/bin/taxonate/main.rs
    ./src/bin/taxonate/app.rs
    ./src/lib.rs
    ./src/languages.rs
    ./src/config.rs

### Supported Languages

You can display a list of the supported languages (in `key: name` format) by
running the following command:

    $ taxonate --list-languages

Where `key` is what you should provide to the `--language` option when
filtering. See the [`data/` directory](/data) for more details on how the
languages are defined.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
