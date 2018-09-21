use std::error;
use std::fmt;

pub struct Token {
    pub value: String,
    pub is_acronym: bool,
}

pub struct Name {
    tokens: Vec<Token>,
    // TODO plurals
}

impl Name {
    pub fn parse(s: &str) -> Result<Self, ParsingError> {
        // TODO handle errors
        // TODO plural
        // TODO acronym
        let tokens = s
            .split_whitespace()
            .map(|s| Token {
                value: s.to_owned(),
                is_acronym: false,
            }).collect();
        Ok(Name { tokens })
    }
    pub fn singular<'a>(&'a self) -> Box<Iterator<Item = &'a Token> + 'a> {
        Box::new(self.tokens.iter())
    }
}

#[derive(Debug, Clone)]
pub struct ParsingError(String);

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for ParsingError {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[test]
fn test_parse_name() {
    let name = Name::parse(" foo  bar ").unwrap();
    assert_eq!(
        name.singular().map(|t| &t.value).collect::<Vec<&String>>(),
        vec!["foo", "bar"],
    );
}
