// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{collections::HashSet, error::Error, ffi::OsString};

use log::debug;

#[derive(Debug)]
pub struct Config {
    pub color: String,
    pub debug: String,
    pub language: Option<String>,
    pub list: bool,
    pub paths: HashSet<OsString>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    debug!("'config': {:?}", config);
    Ok(())
}
