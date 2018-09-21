use std;

use name::Token;

enum Case {
    Lower,
    Upper,
    Title,
}

impl Case {
    fn format(&self, s: &str) -> String {
        match self {
            Case::Lower => s.to_lowercase(),
            Case::Upper => s.to_uppercase(),
            Case::Title => {
                let mut result = String::new();
                let mut chars = s.chars().fuse();
                if let Some(c) = chars.next() {
                    result.extend(c.to_uppercase());
                }
                for c in chars {
                    result.extend(c.to_lowercase());
                }
                result
            }
        }
    }
}

pub trait NamingStyle {
    fn format<'a, I: IntoIterator<Item = &'a Token>>(&self, tokens: I) -> String;
}

pub struct SimpleNamingStyle {
    first_case: Case,
    other_case: Case,
    upper_acronyms: bool,
    separator: std::borrow::Cow<'static, str>,
}

impl NamingStyle for SimpleNamingStyle {
    fn format<'a, I: IntoIterator<Item = &'a Token>>(&self, tokens: I) -> String {
        let formatted_tokens: Vec<String> = tokens
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                match (i, t) {
                    (_, t) if self.upper_acronyms && t.is_acronym => &Case::Upper,
                    (0, _) => &self.first_case,
                    _ => &self.other_case,
                }.format(&t.value)
            }).collect();
        return formatted_tokens.join(&self.separator);
    }
}

pub static STYLES: &[SimpleNamingStyle] = &[
    // snake_case
    SimpleNamingStyle {
        first_case: Case::Lower,
        other_case: Case::Lower,
        upper_acronyms: false,
        separator: std::borrow::Cow::Borrowed("_"),
    },
    // UPPER_CASE
    SimpleNamingStyle {
        first_case: Case::Upper,
        other_case: Case::Upper,
        upper_acronyms: false,
        separator: std::borrow::Cow::Borrowed("_"),
    },
    // camelCase
    SimpleNamingStyle {
        first_case: Case::Lower,
        other_case: Case::Title,
        upper_acronyms: true,
        separator: std::borrow::Cow::Borrowed(""),
    },
    // TitleCase
    SimpleNamingStyle {
        first_case: Case::Title,
        other_case: Case::Title,
        upper_acronyms: true,
        separator: std::borrow::Cow::Borrowed(""),
    },
];

#[test]
fn test_simple_style() {
    let style = SimpleNamingStyle {
        first_case: Case::Lower,
        other_case: Case::Title,
        upper_acronyms: true,
        separator: "_".into(),
    };
    let tokens = vec![
        Token {
            value: "protocol".to_owned(),
            is_acronym: false,
        },
        Token {
            value: "named".to_owned(),
            is_acronym: false,
        },
        Token {
            value: "tcp".to_owned(),
            is_acronym: true,
        },
    ];
    assert_eq!(style.format(&tokens), "protocol_Named_TCP");
}
