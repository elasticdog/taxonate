// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    error::Error,
    io::{self, Write},
    path::Path,
};

use ignore::Walk;
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

    let mut lang_filter: Option<&Language> = None;
    if let Some(key) = config.language() {
        lang_filter = LANGUAGES.languages.get(key);
    }
    info!("applying language filter: {:?}", lang_filter);

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut buffer = io::BufWriter::new(handle);

    for path in config.paths() {
        let walker = Walk::new(path);

        let files = walker
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().expect("no file type").is_file());

        for file in files {
            identify_and_print(
                file.path(),
                config.filename_only(),
                lang_filter,
                buffer.get_mut(),
            )?;
        }
    }

    Ok(())
}

fn identify_and_print<W: Write>(
    file: &Path,
    filename_only: bool,
    lang_filter: Option<&Language>,
    writer: &mut W,
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
            writeln!(writer, "{}", file.display())?;
        } else {
            writeln!(writer, "{}: {}", file.display(), lang_name)?;
        }
    }

    Ok(())
}

#[must_use]
pub fn identify(file: &Path) -> Option<&Language> {
    languages::find_by_interpreter(&file).or_else(|| languages::find_by_glob(&file))
}

fn should_print(lang: Option<&Language>, lang_filter: Option<&Language>) -> bool {
    if lang_filter.is_none() {
        true
    } else {
        lang == lang_filter
    }
}
