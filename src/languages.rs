// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::BTreeMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use glob::Pattern;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::Deserialize;

const LANGUAGES_JSON: &str = include_str!("../data/languages.json");

#[derive(Deserialize)]
pub struct Languages {
    pub languages: BTreeMap<String, Language>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Language {
    pub name: String,
    pub globs: Vec<String>,
    pub interpreters: Vec<String>,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

lazy_static! {
    pub static ref LANGUAGES: Languages = serde_json::from_str(&LANGUAGES_JSON).unwrap();
}

pub(crate) fn find_by_glob(file: &Path) -> Option<&Language> {
    let file_name = file.file_name()?.to_str()?;

    let result = LANGUAGES
        .languages
        .par_iter()
        .find_any(|(_, lang)| matches_any_glob(&lang.globs, file_name));

    match result {
        Some((_, lang)) => Some(&lang),
        None => None,
    }
}

fn matches_any_glob(globs: &[String], str: &str) -> bool {
    globs
        .par_iter()
        .any(|glob| Pattern::new(glob).unwrap().matches(str))
}

pub(crate) fn find_by_interpreter(file: &Path) -> Option<&Language> {
    let result = LANGUAGES
        .languages
        .par_iter()
        .find_any(|(_, lang)| matches_any_interpreter(&lang.interpreters, file));

    match result {
        Some((_, lang)) => Some(&lang),
        None => None,
    }
}

fn matches_any_interpreter(interpreters: &[String], file: &Path) -> bool {
    interpreters
        .par_iter()
        .any(|interpreter| matches_shebang(interpreter, file))
}

#[must_use]
pub fn matches_shebang(interpreter: &str, file: &Path) -> bool {
    match find_shebang_interpreter(file) {
        Some(found) => found == interpreter,
        None => false,
    }
}

#[must_use]
fn find_shebang_interpreter(file: &Path) -> Option<String> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);

    parse_shebang(&line)
}

#[must_use]
fn parse_shebang(line: &str) -> Option<String> {
    // ignore leading whitespace
    let line = line.trim_start();

    if line.starts_with("#!") {
        let mut tokens = line.trim_start_matches("#!").split_whitespace();
        let path = Path::new(tokens.next()?);

        if path.is_absolute() {
            if path.ends_with("env") {
                tokens.next().map(String::from)
            } else {
                // TODO: this conversion chain smells bad
                path.file_name().unwrap().to_str().map(String::from)
            }
        } else {
            None
        }
    } else {
        None
    }
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
