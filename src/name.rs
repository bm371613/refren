use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: String,
    pub is_acronym: bool,
}

pub struct Name {
    tokens: Vec<Token>,
    // TODO plurals
}

impl Name {
    pub fn parse(raw_name: &str) -> Result<Self, ParsingError> {
        match raw_name
            .split_whitespace()
            .map(|raw_token| {
                let any_lower = raw_token.bytes().any(|b| b.is_ascii_lowercase());
                let any_upper = raw_token.bytes().any(|b| b.is_ascii_uppercase());
                if any_lower & any_upper {
                    Err(ParsingError::MixedCaseToken {
                        raw_token: raw_token.to_owned(),
                    })
                } else {
                    Ok(Token {
                        value: raw_token.to_owned(),
                        is_acronym: !any_lower,
                    })
                }
            }).collect()
        {
            Ok(tokens) => Ok(Name { tokens }),
            Err(e) => Err(e),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    pub fn singular<'a>(&'a self) -> Box<Iterator<Item = &'a Token> + 'a> {
        Box::new(self.tokens.iter())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingError {
    MixedCaseToken { raw_token: String },
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::MixedCaseToken { raw_token } => write!(
                f,
                "Mixed case in {:?}. Use upper case for acronyms, lowercase otherwise.",
                raw_token,
            ),
        }
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
fn test_simple() {
    let name = Name::parse(" foo  BAR ").unwrap();
    assert_eq!(
        name.singular().collect::<Vec<&Token>>(),
        vec![
            &Token {
                value: "foo".to_owned(),
                is_acronym: false
            },
            &Token {
                value: "BAR".to_owned(),
                is_acronym: true
            },
        ],
    );
}

#[test]
fn test_mixed_case_error() {
    let error = Name::parse(" foo  Bar ").err().unwrap();
    assert_eq!(
        error,
        ParsingError::MixedCaseToken {
            raw_token: "Bar".to_owned()
        }
    );
}
