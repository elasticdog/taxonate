// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{collections::BTreeMap, path::Path};

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
