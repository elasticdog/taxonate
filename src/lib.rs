// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::HashSet,
    error::Error,
    io::{self, Write},
};

use log::debug;

pub mod languages;

#[derive(Debug, Default)]
pub struct Config {
    color: String,
    debug: String,
    language: Option<String>,
    list_languages: bool,
    paths: HashSet<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            color: "auto".to_owned(),
            debug: "error".to_owned(),
            language: None,
            list_languages: false,
            paths: HashSet::new(),
        }
    }

    /// Set when to output color.
    pub fn color<'a>(&'a mut self, when: String) -> &'a mut Config {
        self.color = when;
        self
    }

    /// Get when to output color.
    pub fn get_color(&self) -> &String {
        &self.color
    }

    /// Set the debug logging level.
    pub fn debug<'a>(&'a mut self, level: String) -> &'a mut Config {
        self.debug = level;
        self
    }

    /// Get the debug logging level.
    pub fn get_debug(&self) -> &String {
        &self.debug
    }

    /// Set the language for filtering output.
    pub fn language<'a>(&'a mut self, lang: Option<String>) -> &'a mut Config {
        self.language = lang;
        self
    }

    /// Get the language for filtering output.
    pub fn get_language(&self) -> &Option<String> {
        &self.language
    }

    /// Set the language for filtering output.
    pub fn list_languages<'a>(&'a mut self, list: bool) -> &'a mut Config {
        self.list_languages = list;
        self
    }

    /// Get the language for filtering output.
    pub fn get_list_languages(&self) -> bool {
        self.list_languages
    }

    /// Add a path to scan for language identification.
    pub fn path<'a>(&'a mut self, path: String) -> &'a mut Config {
        self.paths.insert(path);
        self
    }

    /// Add multiple paths to scan for language idenfication.
    pub fn paths<'a>(&'a mut self, paths: &[String]) -> &'a mut Config {
        for path in paths {
            self.paths.insert(path.to_owned());
        }
        self
    }

    /// Add multiple paths to scan for language idenfication.
    pub fn paths_hashset<'a>(&'a mut self, paths: HashSet<String>) -> &'a mut Config {
        self.paths = paths;
        self
    }

    /// Get the paths to scan for language identification.
    pub fn get_paths(&self) -> &HashSet<String> {
        &self.paths
    }

    pub fn build(&mut self) -> Config {
        let this = std::mem::take(self);
        Config {
            color: this.color,
            debug: this.debug,
            language: this.language,
            list_languages: this.list_languages,
            paths: this.paths,
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    debug!("'config': {:?}", config);

    {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut buffer = io::BufWriter::new(handle);

        for path in &config.paths {
            buffer.write_all(path.as_bytes())?;
            buffer.write_all(b"\n")?;
        }
    } // end scope to unlock stdout and flush

    Ok(())
}
