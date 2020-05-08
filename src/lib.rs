// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    error::Error,
    io::{self, Write},
    path::PathBuf,
};

use log::{debug, info, trace};

pub mod config;
pub mod languages;

use crate::config::Config;
use crate::languages::{Language, LANGUAGES};

/// # Errors
///
/// Will return `Err` if ...TODO...
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    debug!("configuration settings: {:?}", config);

    let filename_only = config.filename_only();

    let mut lang_filter: Option<&Language> = None;
    if let Some(key) = config.language() {
        lang_filter = LANGUAGES.languages.get(key);
    }
    info!("applying language filter: {:?}", lang_filter);

    // TODO: convert directories into recursive lists of files
    let files = config.paths().iter().filter(|f| f.is_file());

    {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut buffer = io::BufWriter::new(handle);

        for file in files {
            identify_and_print(file, filename_only, lang_filter, buffer.get_mut())?;
        }
    } // end scope to unlock stdout and flush

    Ok(())
}

fn identify_and_print<W: Write>(
    file: &PathBuf,
    filename_only: bool,
    lang_filter: Option<&Language>,
    buffer: &mut W,
) -> Result<(), Box<dyn Error>> {
    let lang = identify(file);
    let lang_name = match lang {
        Some(lang) => &lang.name,
        None => "Unknown",
    };

    trace!(
        "file {:?} identified as language {:?}",
        file.display(),
        lang_name
    );

    if should_print(lang, lang_filter) {
        if filename_only {
            writeln!(buffer, "{}", file.display())?;
        } else {
            writeln!(buffer, "{}: {}", file.display(), lang_name)?;
        }
    }

    Ok(())
}

#[must_use]
pub fn identify(file: &PathBuf) -> Option<&Language> {
    languages::find_interpreter_match(&file.as_path())
        .or_else(|| languages::find_glob_match(&file.as_path()))
}

fn should_print(lang: Option<&Language>, lang_filter: Option<&Language>) -> bool {
    if lang_filter.is_none() {
        true
    } else {
        lang == lang_filter
    }
}
