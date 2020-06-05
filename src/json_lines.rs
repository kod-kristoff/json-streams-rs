use std::{
    fs::File,
    io::{self, prelude::*},
    rc::Rc
};

pub struct JsonLinesReader<T: io::BufRead> {
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

impl<T: io::BufRead> Iterator for JsonLinesReader<T> {
    type Item = Rc<String>;

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
            Ok(n) if n > 0 => Some(Rc::clone(&self.buf)),
            _ => None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn create_json_lines_reader() {
        let data = "a\na\n".as_bytes();

        let json_lines_reader = JsonLinesReader {
            reader: io::BufReader::new(data),
            buf: Default::default(),
        };
        for v in json_lines_reader {
            assert_eq!(*v.trim(), "a".to_owned());
        }
    }
}
