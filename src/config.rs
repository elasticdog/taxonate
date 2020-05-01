// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;

#[derive(Debug)]
pub enum Color {
    Always,
    Auto,
    Never,
}

impl Default for Color {
    fn default() -> Self {
        Color::Auto
    }
}

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Error
    }
}

#[derive(Debug, Default)]
pub struct Config {
    color: Color,
    log_level: LogLevel,
    language: Option<String>,
    paths: HashSet<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            color: Color::Auto,
            log_level: LogLevel::Error,
            language: None,
            paths: HashSet::new(),
        }
    }

    /// Set when to output color.
    pub fn color<'a>(&'a mut self, when: Color) -> &'a mut Config {
        self.color = when;
        self
    }

    /// Get when to output color.
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    /// Set the logging level.
    pub fn log_level<'a>(&'a mut self, level: LogLevel) -> &'a mut Config {
        self.log_level = level;
        self
    }

    /// Get the logging level.
    pub fn get_log_level(&self) -> &LogLevel {
        &self.log_level
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

    // FIXME: Do I need a separate ConfigBuilder type?
    pub fn build(&mut self) -> Config {
        let this = std::mem::take(self);
        Config {
            color: this.color,
            log_level: this.log_level,
            language: this.language,
            paths: this.paths,
        }
    }
}
