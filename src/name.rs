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
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    pub fn singular<'a>(&'a self) -> Box<Iterator<Item = &'a Token> + 'a> {
        Box::new(self.tokens.iter())
    }
}

#[derive(Debug, Clone)]
pub struct ParsingError;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrectly formatted name")
    }
}

impl error::Error for ParsingError {
    fn description(&self) -> &str {
        "Incorrectly formatted name"
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
