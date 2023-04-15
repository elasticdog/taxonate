// SPDX-License-Identifier: MIT OR Apache-2.0

use glob::Pattern;
use std::{collections::BTreeMap, fmt};

use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

const LANGUAGES_JSON: &str = include_str!("../data/languages.json");

#[derive(Deserialize)]
pub struct Languages {
    pub languages: BTreeMap<String, Language>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Language {
    pub name: String,
    pub globs: Vec<Pattern>,
    pub interpreters: Vec<String>,
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawLanguage {
            name: String,
            globs: Vec<String>,
            interpreters: Vec<String>,
        }

        let raw_language = RawLanguage::deserialize(deserializer)?;

        let globs = raw_language
            .globs
            .into_iter()
            .map(|glob| Pattern::new(&glob).unwrap())
            .collect();

        Ok(Language {
            name: raw_language.name,
            globs,
            interpreters: raw_language.interpreters,
        })
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub static LANGUAGES: Lazy<Languages> = Lazy::new(|| serde_json::from_str(LANGUAGES_JSON).unwrap());
