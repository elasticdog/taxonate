// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    collections::HashSet,
    error::Error,
    io::{self, Write},
};

use log::debug;

pub mod languages;

#[derive(Debug)]
pub struct Config {
    pub color: String,
    pub debug: String,
    pub language: Option<String>,
    pub list_languages: bool,
    pub paths: HashSet<String>,
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
