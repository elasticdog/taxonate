// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{collections::BTreeMap, fmt};

use lazy_static::lazy_static;
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
    pub static ref LANGUAGES: Languages = serde_json::from_str(LANGUAGES_JSON).unwrap();
}
