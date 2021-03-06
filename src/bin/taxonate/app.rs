// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::HashSet,
    env,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, ArgMatches};

use taxonate::config::{Color, Config, LogLevel};

pub fn build() -> App<'static, 'static> {
    let color = env::var("TAXONATE_COLOR").unwrap_or_else(|_| "auto".to_owned());
    let color_app_setting = match color.as_str() {
        "always" => AppSettings::ColorAlways,
        "never" => AppSettings::ColorNever,
        _ => AppSettings::ColorAuto,
    };

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::AllowInvalidUtf8)
        .setting(AppSettings::ColoredHelp)
        .setting(color_app_setting)
        .about(
            "Identify and filter files based on their programming language.\n\n\
            Use '--help' instead of '-h' to see a more detailed version of the \
            help text.",
        )
        .long_about("Identify and filter files based on their programming language.")
        .arg(
            Arg::with_name("filename_only")
                .help("Suppresses display of the identified language")
                .long_help(
                    "Suppresses normal output; only displays the file name and \
                    not the identified programming language",
                )
                .short("f")
                .long("filename-only"),
        )
        .arg(
            Arg::with_name("list_languages")
                .help("Lists supported programming languages")
                .long_help(
                    "Displays a list of supported programming languages for \
                    filtering output",
                )
                .short("L")
                .long("list-languages"),
        )
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
                .help("Filters output by programming language")
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
            Arg::with_name("PATH")
                .help("File or directory to identify. Use '-' for standard input.")
                .long_help(
                    "A file or directory to identify. Directories will have \
                    all files identified recursively. Use a dash ('-') to \
                    read from standard input.",
                )
                .multiple(true),
        )
}

pub fn config_from(matches: &ArgMatches) -> Config {
    // unwrap is safe since we specify a default
    let color = match matches.value_of("color").unwrap() {
        "auto" => Color::Auto,
        "always" => Color::Always,
        "never" => Color::Never,
        _ => unreachable!(),
    };

    let filename_only = matches.is_present("filename_only");

    // unwrap is safe since we specify a default
    let log_level = match matches.value_of("debug").unwrap() {
        "error" => LogLevel::Error,
        "warning" => LogLevel::Warning,
        "info" => LogLevel::Info,
        "debug" => LogLevel::Debug,
        "trace" => LogLevel::Trace,
        _ => unreachable!(),
    };

    let language = matches.value_of("language").map(String::from);

    let mut paths: HashSet<PathBuf> = matches
        .values_of_os("PATH")
        .unwrap_or_default()
        .map(PathBuf::from)
        .collect();

    // include paths from STDIN, if explicitly requested
    if paths.remove(Path::new("-")) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            paths.insert(PathBuf::from(line.unwrap()));
        }
    }

    if paths.is_empty() {
        // default to the current working directory
        paths.insert(PathBuf::from("."));
    }

    Config::new()
        .set_color(color)
        .set_filename_only(filename_only)
        .set_log_level(log_level)
        .set_language(language)
        .set_paths(paths)
}
