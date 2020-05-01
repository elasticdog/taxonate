// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

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
