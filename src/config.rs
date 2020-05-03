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
    #[must_use]
    pub fn new() -> Config {
        Config {
            color: Color::Auto,
            log_level: LogLevel::Error,
            language: None,
            paths: HashSet::new(),
        }
    }

    /// Get when to output color.
    #[must_use]
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// Set when to output color.
    #[must_use]
    pub fn set_color(mut self, val: Color) -> Config {
        self.color = val;
        self
    }

    /// Get the logging level.
    #[must_use]
    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    /// Set the logging level.
    #[must_use]
    pub fn set_log_level(mut self, val: LogLevel) -> Config {
        self.log_level = val;
        self
    }

    /// Get the language for filtering output.
    #[must_use]
    pub fn language(&self) -> &Option<String> {
        &self.language
    }

    /// Set the language for filtering output.
    #[must_use]
    pub fn set_language(mut self, val: Option<String>) -> Config {
        self.language = val;
        self
    }

    /// Get the paths to scan for language identification.
    #[must_use]
    pub fn paths(&self) -> &HashSet<String> {
        &self.paths
    }

    /// Add a path to scan for language identification.
    #[must_use]
    pub fn add_path(mut self, val: String) -> Config {
        self.paths.insert(val);
        self
    }

    /// Add multiple paths to scan for language idenfication.
    #[must_use]
    pub fn set_paths(mut self, val: HashSet<String>) -> Config {
        self.paths = val;
        self
    }
}
