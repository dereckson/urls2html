//! # urls2html
//!
//! A tool to quickly prepare a list of URLs in text and convert it into HTML.

extern crate urls2html;
extern crate argparse;

use std::process;

use argparse::ArgumentParser;
use argparse::StoreFalse;
use argparse::Store;

use urls2html::Config;
use urls2html::run;

fn main() {

    //
    // Parse configuration
    //

    let config = parse_config().unwrap_or_else(|error_code| {
        process::exit(error_code);
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

fn parse_config() -> Result<Config, i32> {
    let mut config = Config::new();

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("Prepare a list of URLs.");

        parser.refer(&mut config.filename)
            .add_argument("filename", Store, "The name of the file to read URLs from");

        parser.refer(&mut config.full_html_document)
            .add_option(&["-s", "--short"], StoreFalse, "Only print the <ul> links block");

        parser.refer(&mut config.title)
            .add_option(&["-t", "--title"], Store, "Set the document title");

        parser.parse_args()?;
    }

    Ok(config)
}
