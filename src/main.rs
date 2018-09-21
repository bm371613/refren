mod name;
mod replace;
mod style;

use std::io;

use name::Name;
use replace::default_replacer;
use style::NamingStyle;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).take(2).collect();
    let old_name = Name::parse(&args[0]).unwrap();
    let new_name = Name::parse(&args[1]).unwrap();
    let replacer = default_replacer(
        style::STYLES
            .iter()
            .map(|s| (s.format(old_name.singular()), s.format(new_name.singular()))),
    );
    replacer.transform(&mut io::stdin(), &mut io::stdout())
}
