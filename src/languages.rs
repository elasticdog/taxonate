// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use lazy_static::lazy_static;
use log::debug;
use serde::Deserialize;

const LANGUAGES_JSON: &str = include_str!("../data/languages.json");

#[derive(Deserialize)]
struct Languages {
    languages: BTreeMap<String, Language>,
}

#[derive(Deserialize)]
struct Language {
    name: String,
    globs: Vec<String>,
    interpreters: Vec<String>,
}

lazy_static! {
    static ref LANGUAGES: Languages = serde_json::from_str(&LANGUAGES_JSON).unwrap();
}

pub fn list() {
    debug!("listing supported programming languages");
    for (key, lang) in &LANGUAGES.languages {
        println!("{}: {}", key, lang.name);
    }
}
