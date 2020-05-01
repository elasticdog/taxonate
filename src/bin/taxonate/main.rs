// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    io::{self, Write},
    process,
};

use log::{debug, error};

use taxonate::languages::LANGUAGES;

mod app;

fn main() {
    // parse command line arguments
    let matches = app::build().get_matches();

    // initialize logging
    let env = env_logger::Env::default()
        .filter_or("TAXONATE_DEBUG", matches.value_of("debug").unwrap())
        .write_style_or("TAXONATE_COLOR", matches.value_of("color").unwrap());

    env_logger::init_from_env(env);

    if matches.is_present("list_languages") {
        list_languages().ok();
        process::exit(0);
    }

    let config = app::config_from(&matches);

    let result = taxonate::run(&config);

    match result {
        Ok(()) => {
            process::exit(0);
        }
        Err(e) => {
            error!("Application error: {}", e);
            process::exit(1);
        }
    }
}

fn list_languages() -> io::Result<()> {
    debug!("listing supported programming languages");

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut buffer = io::BufWriter::new(handle);

    for (key, lang) in &LANGUAGES.languages {
        writeln!(buffer, "{}: {}", key, lang.name)?;
    }

    Ok(())
}
