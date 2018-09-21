use std::collections::HashMap;
use std::io;
use std::iter::{once, FromIterator};

pub trait Replacer {
    fn transform(&self, reader: &mut io::Read, writer: &mut io::Write) -> io::Result<()>;
}

struct SimpleReplacer {
    map: HashMap<String, String>,
}

impl SimpleReplacer {
    fn from_map<I: IntoIterator<Item = (String, String)>>(map: I) -> Self {
        SimpleReplacer {
            map: HashMap::from_iter(map),
        }
    }
}

impl Replacer for SimpleReplacer {
    fn transform(&self, reader: &mut io::Read, writer: &mut io::Write) -> io::Result<()> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let raw = content.as_bytes();
        let char_ixs: Vec<usize> = content
            .char_indices()
            .map(|(ix, _)| ix)
            .chain(once(raw.len()))
            .collect();
        let mut i = 0;
        while i < content.len() {
            match self.map.iter().find(|(k, _)| content[i..].starts_with(*k)) {
                Some((k, v)) => {
                    writer.write(v.as_bytes())?;
                    i += k.len();
                }
                None => {
                    writer.write(&raw[char_ixs[i]..char_ixs[i + 1]])?;
                    i += 1;
                }
            }
        }
        Ok(())
    }
}

#[test]
fn test_simple_replacer() {
    let replacer = SimpleReplacer::from_map(vec![
        ("aa".to_owned(), "aaa".to_owned()),
        (":)".to_owned(), "😊".to_owned()),
        ("😺".to_owned(), "a cat".to_owned()),
    ]);
    let src: Vec<u8> = "Whaaat? :) 😺".bytes().collect();
    let mut dst: Vec<u8> = Vec::new();
    assert_eq!(replacer.transform(&mut &src[..], &mut dst).unwrap(), ());
    assert_eq!(String::from_utf8(dst).unwrap(), "Whaaaat? 😊 a cat");
}

pub fn default_replacer<I: IntoIterator<Item = (String, String)>>(map: I) -> Box<Replacer> {
    Box::new(SimpleReplacer::from_map(map))
}
