// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use glob::Pattern;
use ignore::Walk;
use log::{debug, info, trace};
use rayon::prelude::*;

pub mod config;
pub mod languages;

use crate::config::Config;
use crate::languages::{Language, LANGUAGES};

/// # Errors
///
/// Will return `Err` if ...TODO...
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    debug!("configuration settings: {:?}", config);

    let mut lang_filter: Option<&Language> = None;
    if let Some(key) = config.language() {
        lang_filter = LANGUAGES.languages.get(key);
    }
    info!("applying language filter: {:?}", lang_filter);

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut buffer = io::BufWriter::new(handle);

    for path in config.paths() {
        let walker = Walk::new(path);

        let files = walker
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().expect("no file type").is_file());

        for file in files {
            identify_and_print(
                file.path(),
                config.filename_only(),
                lang_filter,
                buffer.get_mut(),
            )?;
        }
    }

    Ok(())
}

fn identify_and_print<W: Write>(
    file: &Path,
    filename_only: bool,
    lang_filter: Option<&Language>,
    writer: &mut W,
) -> Result<(), Box<dyn Error>> {
    let lang = identify(file);

    let lang_name = match lang {
        Some(lang) => &lang.name,
        None => "Unknown",
    };

    trace!(
        "file {:?} identified as language {:?}",
        file.display(),
        lang_name
    );

    if should_print(lang, lang_filter) {
        if filename_only {
            writeln!(writer, "{}", file.display())?;
        } else {
            writeln!(writer, "{}: {}", file.display(), lang_name)?;
        }
    }

    Ok(())
}

#[must_use]
pub fn identify(file: &Path) -> Option<&Language> {
    find_lang_by_interpreter(file).or_else(|| find_lang_by_glob(file))
}

fn find_lang_by_interpreter(file: &Path) -> Option<&Language> {
    let result = LANGUAGES
        .languages
        .par_iter()
        .find_any(|(_, lang)| matches_any_interpreter(&lang.interpreters, file));

    match result {
        Some((_, lang)) => Some(lang),
        None => None,
    }
}

fn matches_any_interpreter(interpreters: &[String], file: &Path) -> bool {
    interpreters
        .par_iter()
        .any(|interpreter| Some(interpreter.clone()) == read_interpreter(file))
}

#[must_use]
fn read_interpreter(file: &Path) -> Option<String> {
    let file = File::open(file).ok()?;
    let buf = BufReader::new(file);
    let line = buf.lines().next()?.ok()?;
    parse_shebang(&line)
}

#[must_use]
fn parse_shebang(line: &str) -> Option<String> {
    // ignore leading whitespace
    let line = line.trim_start();
    if !line.starts_with("#!") {
        return None;
    }

    let mut tokens = line.trim_start_matches("#!").split_whitespace();
    let path = Path::new(tokens.next()?);

    if !path.is_absolute() {
        return None;
    }

    if path.ends_with("env") {
        return tokens.next().map(String::from);
    }

    path.file_name()?.to_os_string().into_string().ok()
}

fn find_lang_by_glob(file: &Path) -> Option<&Language> {
    let file_name = file.file_name()?.to_str()?;

    LANGUAGES
        .languages
        .par_iter()
        .find_any(|(_, lang)| matches_any_glob(&lang.globs, file_name))
        .map(|(_, lang)| lang)
}

fn matches_any_glob(globs: &[Pattern], str: &str) -> bool {
    globs.par_iter().any(|glob| glob.matches(str))
}

fn should_print(lang: Option<&Language>, lang_filter: Option<&Language>) -> bool {
    lang_filter.map_or(true, |filter| lang == Some(filter))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shebang_typical() {
        assert_eq!(Some(String::from("bash")), parse_shebang("#!/bin/bash"));
    }

    #[test]
    fn shebang_with_argument() {
        assert_eq!(Some(String::from("bash")), parse_shebang("#!/bin/bash -x"));
    }

    #[test]
    fn shebang_with_env_indirection() {
        assert_eq!(
            Some(String::from("bash")),
            parse_shebang("#!/usr/bin/env bash")
        );
    }

    #[test]
    fn shebang_with_leading_whitespace() {
        assert_eq!(Some(String::from("bash")), parse_shebang(" #!/bin/bash"));
    }

    #[test]
    fn shebang_with_inner_whitespace() {
        assert_eq!(Some(String::from("bash")), parse_shebang("#! /bin/bash"));
    }

    #[test]
    fn shebang_without_absolute_path() {
        assert_eq!(None, parse_shebang("#!bash"));
    }

    #[test]
    fn shebang_without_env_argument() {
        assert_eq!(None, parse_shebang("#!/usr/bin/env"));
    }

    #[test]
    fn shebang_without_interpreter() {
        assert_eq!(None, parse_shebang("#!"));
    }

    #[test]
    fn shebang_without_shebang() {
        assert_eq!(None, parse_shebang("bash"));
    }
}
