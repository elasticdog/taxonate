// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{collections::HashSet, error::Error, ffi::OsString};

use log::debug;

pub struct Config {
    pub color: String,
    pub debug: String,
    pub language: Option<String>,
    pub list: bool,
    pub paths: HashSet<OsString>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    debug!("'color' WHEN: {:?}", &config.color);
    debug!("'debug' LEVEL: {:?}", &config.debug);
    debug!(
        "'language' LANGUAGE: {:?}",
        &config.language.unwrap_or_default()
    );
    debug!("'list': {:?}", &config.list);
    debug!("'paths': {:?}", &config.paths);

    Ok(())
}
