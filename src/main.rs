//! # urls2html
//!
//! A tool to quickly prepare a list of URLs in text and convert it into HTML.

extern crate urls2html;

use std::env;
use std::env::Args;
use std::process;

use urls2html::Config;
use urls2html::run;

fn main() {

    //
    // Parse configuration
    //

    let config = parse_config(env::args()).unwrap_or_else(|error_message| {
        eprintln!("{}", error_message);
        eprintln!("Usage: urls2html <source file> [title]");

        process::exit(1);
    });

    //
    // Run task
    //

    if let Err(error) = run(config) {
        eprintln!("{}", error);

        process::exit(1);
    }

    process::exit(0);
}

fn parse_config(mut args: Args) -> Result<Config, &'static str> {
    args.next(); // command name

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("You must specify a file to grab URLs from."),
    };

    let title = match args.next() {
        Some(arg) => arg,
        None => String::from(urls2html::DEFAULT_TITLE),
    };

    Ok(Config {
        filename,
        title,
        full_html_document: true,
    })
}
