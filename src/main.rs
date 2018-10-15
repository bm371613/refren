extern crate clap;

mod name;
mod style;
mod transform;

use std::fs::File;
use std::io;
use std::io::Read;

use clap::{App, Arg, ArgMatches};

use name::Name;
use style::NamingStyle;
use transform::default_transformer;

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("OLD_NAME")
                .help("Old name")
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("NEW_NAME")
                .help("New name")
                .required(true)
                .index(2),
        ).arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Modify the given file instead of using standard input and output")
                .takes_value(true),
        ).after_help(
            "\
             Both names should be space-separated lists of tokens. \n\
             Use uppercase for acronyms, lowercase otherwise, eg. \"bad HTTP response\"",
        );

    run_command(app.get_matches()).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    });
}

fn run_command(arg_matches: ArgMatches) -> Result<(), Box<std::error::Error>> {
    let old_name = Name::parse(arg_matches.value_of("OLD_NAME").unwrap())?;
    let new_name = Name::parse(arg_matches.value_of("NEW_NAME").unwrap())?;
    if old_name.is_empty() {
        return Err("OLD_NAME is empty".into());
    }
    let transformer = default_transformer(
        style::STYLES
            .iter()
            .map(|s| (s.format(old_name.singular()), s.format(new_name.singular()))),
    );
    match arg_matches.value_of("file") {
        Some(file_name) => {
            // TODO optionally use a temporary file instead of memory
            let mut content: Vec<u8> = Vec::new();
            File::open(file_name)?.read_to_end(&mut content)?;
            transformer.transform(&mut &content[..], &mut File::create(file_name)?)
        }
        None => transformer.transform(&mut io::stdin(), &mut io::stdout()),
    }?;
    Ok(())
}
