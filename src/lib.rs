// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    error::Error,
    io::{self, Write},
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

    {
        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut buffer = io::BufWriter::new(handle);

        for path in config.get_paths().iter() {
            buffer.write_all(path.as_bytes())?;
            buffer.write_all(b"\n")?;
        }
    } // end scope to unlock stdout and flush

    Ok(())
}
