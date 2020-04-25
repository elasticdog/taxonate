// SPDX-License-Identifier: MIT OR Apache-2.0

use std::error::Error;
use std::io::{self, Write};

use crate::config::Config;
use bstr::io::BufReadExt;
use log::debug;

pub mod config;

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
