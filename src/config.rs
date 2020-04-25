// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{crate_authors, crate_name, crate_version, App, Arg};

pub struct Config {
    pub color: String,
    pub debug: String,
    pub language: Option<String>,
    pub list: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let matches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(
                "Identify and filter files based on their programming language.\n\
                \n\
                Use '--help' instead of '-h' to see a more detailed version of the \
                help text.",
            )
            .long_about("Identify and filter files based on their programming language.")
            .arg(
                Arg::with_name("color")
                    .help("Specifies when to use colored output")
                    .short("c")
                    .long("color")
                    .takes_value(true)
                    .value_name("WHEN")
                    .possible_values(&["auto", "always", "never"])
                    .env("TAXONATE_COLOR")
                    .default_value("auto"),
            )
            .arg(
                Arg::with_name("debug")
                    .help("Adjusts the log level for debugging")
                    .short("d")
                    .long("debug")
                    .takes_value(true)
                    .value_name("LEVEL")
                    .possible_values(&["error", "warning", "info", "debug", "trace"])
                    .env("TAXONATE_DEBUG")
                    .default_value("error"),
            )
            .arg(
                Arg::with_name("language")
                    .help("Outputs files identified as the given language")
                    .long_help(
                        "Filters output to only show files identified as the given \
                        programming language",
                    )
                    .short("l")
                    .long("language")
                    .takes_value(true)
                    .value_name("LANGUAGE")
                    .env("TAXONATE_LANGUAGE"),
            )
            .arg(
                Arg::with_name("list")
                    .help("Lists supported programming languages")
                    .long_help(
                        "Displays a list of supported programming languages for \
                        filtering output",
                    )
                    .short("L")
                    .long("list-languages"),
            )
            .arg(
                Arg::with_name("PATH")
                    .help("File or directory to identify. Use '-' for standard input.")
                    .long_help(
                        "A file or directory to identify. Directories will have \
                        all files identified recursively. Use a dash ('-') or no \
                        argument at all to read from standard input.",
                    )
                    .multiple(true),
            )
            .get_matches();

        // unwrap is safe when an arg is required or we specify a default
        let color = matches.value_of("color").unwrap().to_string();
        let debug = matches.value_of("debug").unwrap().to_string();
        let language = matches.value_of("language").map(String::from);
        let list = matches.is_present("list");

        Ok(Config {
            color,
            debug,
            language,
            list,
        })
    }
}
