extern crate clap;

mod name;
mod style;
mod transform;

use std::io;
use std::process;

use clap::{App, Arg};

use name::Name;
use style::NamingStyle;
use transform::default_transformer;

fn print_error_and_exit(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1);
}

fn main() {
    let matches = App::new("rr: refactor/rename")
        .version("0.0")
        .author("Bartosz Marcinkowski <bm371613@gmail.com>")
        .about("Facilitates refactoring/renaming")
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
        ).get_matches();

    let old_name = Name::parse(matches.value_of("OLD_NAME").unwrap())
        .unwrap_or_else(|e| print_error_and_exit(&format!("OLD_NAME parsing error: {}", e)));
    let new_name = Name::parse(matches.value_of("NEW_NAME").unwrap())
        .unwrap_or_else(|e| print_error_and_exit(&format!("NEW_NAME parsing error: {}", e)));
    if old_name.is_empty() {
        print_error_and_exit("OLD_NAME is empty")
    }
    let transformer = default_transformer(
        style::STYLES
            .iter()
            .map(|s| (s.format(old_name.singular()), s.format(new_name.singular()))),
    );
    transformer
        .transform(&mut io::stdin(), &mut io::stdout())
        .unwrap_or_else(|e| print_error_and_exit(&e.to_string()));
}
