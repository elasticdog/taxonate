// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use glob::Pattern;
use lazy_static::lazy_static;
use serde::Deserialize;

const LANGUAGES_JSON: &str = include_str!("../data/languages.json");

#[derive(Deserialize)]
pub struct Languages {
    pub languages: BTreeMap<String, Language>,
}

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
    pub globs: Vec<String>,
    pub interpreters: Vec<String>,
}

lazy_static! {
    pub static ref LANGUAGES: Languages = serde_json::from_str(&LANGUAGES_JSON).unwrap();
}

pub(crate) fn find_glob_match(file: &Path) -> Option<String> {
    let result = LANGUAGES
        .languages
        .iter()
        .find(|(_, lang)| matches_glob(&lang.globs, file));

    match result {
        Some((_, lang)) => Some(lang.name.to_owned()),
        None => None,
    }
}

fn matches_glob(globs: &[String], file: &Path) -> bool {
    globs
        .iter()
        .any(|glob| Pattern::new(glob).unwrap().matches_path(file))
}

#[must_use]
pub fn parse_shebang(line: &str) -> Option<String> {
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
    fn can_parse_shebang() {
        assert_eq!(Some(String::from("bash")), parse_shebang("#!/bin/bash"));
    }

    #[test]
    fn can_parse_shebang_with_leading_whitespace() {
        assert_eq!(Some(String::from("bash")), parse_shebang(" #!/bin/bash"));
    }

    #[test]
    fn can_parse_shebang_with_inner_whitespace() {
        assert_eq!(Some(String::from("bash")), parse_shebang("#! /bin/bash"));
    }

    #[test]
    fn interpreter_arguments_are_ignored() {
        assert_eq!(Some(String::from("bash")), parse_shebang("#!/bin/bash -x"));
    }

    #[test]
    fn can_parse_shebang_with_env() {
        assert_eq!(
            Some(String::from("bash")),
            parse_shebang("#!/usr/bin/env bash")
        );
    }

    #[test]
    fn shebang_with_env_has_argument() {
        assert_eq!(None, parse_shebang("#!/usr/bin/env"));
    }

    #[test]
    fn shebang_interpreter_path_is_absolute() {
        assert_eq!(None, parse_shebang("#!usr/bin/bash"));
        assert_eq!(None, parse_shebang("#!../bin/bash"));
    }

    #[test]
    fn shebang_missing_interpreter() {
        assert_eq!(None, parse_shebang("#!"));
        assert_eq!(None, parse_shebang("#! bash"));
    }
}
