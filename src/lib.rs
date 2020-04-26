// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    error::Error,
    io::{self, Write},
};

use bstr::io::BufReadExt;
use log::debug;

pub struct Config {
    pub color: String,
    pub debug: String,
    pub language: Option<String>,
    pub list: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    debug!("'color' WHEN: {:?}", &config.color);
    debug!("'debug' LEVEL: {:?}", &config.debug);
    debug!(
        "'language' LANGUAGE: {:?}",
        &config.language.unwrap_or_default()
    );
    debug!("'list': {:?}", &config.list);
    // debug!(
    //     "'PATH' values: {:?}",
    //     matches
    //         .values_of("PATH")
    //         .map(|vals| vals.collect::<Vec<_>>())

    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());

    stdin.lock().for_byte_line_with_terminator(|line| {
        stdout.write_all(line)?;
        Ok(true)
    })?;
    Ok(())
}
