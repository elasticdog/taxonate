// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    error::Error,
    io::{self, Write},
    path::PathBuf,
};

use log::debug;

pub mod config;
pub mod languages;

use crate::config::Config;

/// # Errors
///
/// Will return `Err` if ...TODO...
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    debug!("'config': {:?}", config);

    // TODO: convert directories into recursive lists of files
    let files = config.paths().iter().filter(|f| f.is_file());

    {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut buffer = io::BufWriter::new(handle);

        for file in files {
            writeln!(buffer, "{}: {:?}", file.display(), identify(file))?;
        }
    } // end scope to unlock stdout and flush

    Ok(())
}

#[must_use]
pub fn identify(file: &PathBuf) -> Option<String> {
    languages::find_interpreter_match(&file.as_path())
        .or_else(|| languages::find_glob_match(&file.as_path()))
}
