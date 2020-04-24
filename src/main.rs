// SPDX-License-Identifier: MIT OR Apache-2.0

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use std::error::Error;
use std::io::{self, Write};

use bstr::{io::BufReadExt, ByteSlice};
use clap::{App, Arg};
use env_logger::Env;

fn main() -> Result<(), Box<dyn Error>> {
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

    // initialize logging
    let env = Env::default()
        .filter_or("TAXONATE_DEBUG", matches.value_of("debug").unwrap())
        .write_style_or("TAXONATE_COLOR", matches.value_of("color").unwrap());

    env_logger::init_from_env(env);

    debug!("'color' WHEN: {:?}", matches.value_of("color").unwrap());
    debug!(
        "'debug' LOG_LEVEL: {:?}",
        matches.value_of("debug").unwrap()
    );
    debug!(
        "'PATH' values: {:?}",
        matches
            .values_of("PATH")
            .map(|vals| vals.collect::<Vec<_>>())
    );

    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());

    stdin.lock().for_byte_line_with_terminator(|line| {
        if line.contains_str("Dimension") {
            stdout.write_all(line)?;
        }
        Ok(true)
    })?;
    Ok(())
}
