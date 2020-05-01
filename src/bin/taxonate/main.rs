// SPDX-License-Identifier: MIT OR Apache-2.0

use std::process;

mod args;

fn main() {
    let config = match args::parse() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Config error: {}", e);
            process::exit(1);
        }
    };

    // initialize logging
    let env = env_logger::Env::default()
        .filter_or("TAXONATE_DEBUG", config.get_debug())
        .write_style_or("TAXONATE_COLOR", config.get_color());

    env_logger::init_from_env(env);

    if config.get_list_languages() {
        taxonate::languages::list();
        process::exit(0);
    }

    let result = taxonate::run(&config);

    match result {
        Ok(()) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
