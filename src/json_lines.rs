use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;

pub struct JsonLinesReader {
    reader: io::BufReader<File>,
    buf: Rc<String>,
}

fn new_buf() -> Rc<String> {
    Rc::new(String::with_capacity(1024))
}

impl JsonLinesReader {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let buf = new_buf();

        Ok(Self { reader, buf })
    }
}

impl Iterator for JsonLinesReader {
    type Item = io::Result<Rc<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf = match Rc::get_mut(&mut self.buf) {
            Some(buf) => {
                buf.clear();
                buf
            }
            None => {
                self.buf = new_buf();
                Rc::make_mut(&mut self.buf)
            }
        };

        self.reader
            .read_line(buf)
            .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
            .transpose()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_json_lines_reader() {
        let json_lines_reader = JsonLinesReader::open("test.jsonl");
        assert_eq!(2 + 2, 4);
    }
}
