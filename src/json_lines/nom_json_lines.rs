use crate::{parsers::streaming::json_value, JsonValue};
use std::{
    fs::File,
    io::{self, prelude::*},
    rc::Rc,
};

pub struct NomJsonLinesIterator {
    pub reader: Box<dyn io::BufRead>,
    pub buf: Rc<String>,
}

fn new_buf() -> Rc<String> {
    Rc::new(String::with_capacity(1024))
}

impl NomJsonLinesIterator {
    pub fn open(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = Box::new(io::BufReader::new(file));
        let buf = new_buf();
        Ok(Self { reader, buf })
    }
}

impl Iterator for NomJsonLinesIterator {
    type Item = JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer: &mut String = match Rc::get_mut(&mut self.buf) {
            Some(buf) => {
                // buf.clear();
                buf
            }
            None => {
                // self.buf = new_buf();
                self.buf = Default::default();
                Rc::get_mut(&mut self.buf).unwrap()
            }
        };

        buffer.clear();
        // self.reader
        //     .read_line(buf)
        //     .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
        // .transpose()
        match self.reader.read_line(buffer) {
            Ok(n) if n > 0 => {
                let row = Rc::clone(&self.buf);
                match json_value(&row.as_bytes()) {
                    Ok((rem, value)) => {
                        log::debug!("remaining: {:?}", rem);
                        Some(value)
                    }
                    Err(err) => panic!("error: {}", err),
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_json_lines_reader() {
        let data = "{\"a\":1}\n{\"a\":1}\n".as_bytes();

        let json_lines_reader = NomJsonLinesIterator {
            reader: Box::new(io::BufReader::new(data)),
            buf: Default::default(),
        };
        let expected = JsonValue::Object(
            [(String::from("a"), JsonValue::Num(1.0))]
                .into_iter()
                .collect(),
        );
        for v in json_lines_reader {
            assert_eq!(v, expected);
        }
    }
}
