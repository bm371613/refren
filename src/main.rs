extern crate clap;

mod name;
mod replace;
mod style;

use std::io;

use clap::{App, Arg};

use name::Name;
use replace::default_replacer;
use style::NamingStyle;

fn main() -> io::Result<()> {
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

    let old_name = Name::parse(matches.value_of("OLD_NAME").unwrap()).unwrap();
    let new_name = Name::parse(matches.value_of("NEW_NAME").unwrap()).unwrap();
    let replacer = default_replacer(
        style::STYLES
            .iter()
            .map(|s| (s.format(old_name.singular()), s.format(new_name.singular()))),
    );
    replacer.transform(&mut io::stdin(), &mut io::stdout())
}
