extern crate clap;

mod name;
mod style;
mod transform;

use std::fs::File;
use std::io;
use std::io::Read;

use clap::{App, Arg};

use name::Name;
use style::NamingStyle;
use transform::{default_transformer, Transform};

fn main() {
    run_command(std::env::args_os()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}

fn run_command(args_os: std::env::ArgsOs) -> Result<(), Box<std::error::Error>> {
    let arg_matches = build_app().get_matches_from_safe(args_os)?;

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
    match arg_matches.values_of("FILES") {
        Some(file_names) => {
            for file_name in file_names {
                if let Err(e) = process_file(&*transformer, file_name) {
                    return Err(format!("{}: {}", file_name, e).into());
                }
            }
        }
        None => transformer.transform(&mut io::stdin(), &mut io::stdout())?,
    };
    Ok(())
}

fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
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
            Arg::with_name("FILES")
                .help("Modify given files instead of using standard input and output")
                .multiple(true)
                .index(3),
        ).after_help(
            "\
             Both names should be space-separated lists of tokens. \n\
             Use uppercase for acronyms, lowercase otherwise, eg. \"bad HTTP response\"",
        )
}

fn process_file(transformer: &Transform, file_name: &str) -> io::Result<()> {
    // TODO optionally use a temporary file instead of memory
    let mut content: Vec<u8> = Vec::new();
    File::open(file_name)?.read_to_end(&mut content)?;
    transformer.transform(&mut &content[..], &mut File::create(file_name)?)
}
