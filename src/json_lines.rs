use serde_json::{Error, Value};
use std::{
    fs,
    io::{self, prelude::*},
    rc::Rc,
};
use simple_lines::ReadExt;

pub struct JsonLinesReader<T> {
    pub reader: T,
    pub buf: Rc<String>,
}

fn new_buf() -> Rc<String> {
    Rc::new(String::with_capacity(1024))
}

// impl<T: io::BufRead> JsonLinesReader<T> {
//     pub fn open(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
//         let file = File::open(path)?;
//         let reader = io::BufReader::new(file);
//         let buf = new_buf();
//         Ok(Self { reader, buf })
//     }
// }

pub fn load_from_file(path: impl AsRef<std::path::Path>) -> io::Result<JsonLinesIterator> 
{
    let lines = Box::new(fs::File::open(path)?.lines_rc());
    Ok(JsonLinesIterator { lines })
}

pub struct JsonLinesIterator {
    lines: Box<dyn Iterator<Item = Result<Rc<String>, simple_lines::Error<Rc<String>>>>>,
}

impl Iterator for JsonLinesIterator {
    type Item = Result<Rc<String>, simple_lines::Error<Rc<String>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Ok(line) => {

            } 
        }
    }
}
impl<T: io::BufRead> Iterator for JsonLinesReader<T> {
    type Item = Value;

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
                match serde_json::from_str(&row) {
                    Ok(value) => Some(value),
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
    use serde_json::json;

    #[test]
    fn create_json_lines_reader() {
        let data = "{\"a\":1}\n{\"a\":1}\n".as_bytes();

        let json_lines_reader = JsonLinesReader {
            reader: io::BufReader::new(data),
            buf: Default::default(),
        };
        let expected = json!({
            "a": 1
        });
        for v in json_lines_reader {
            assert_eq!(v, expected);
        }
    }
}
