use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct JsonLinesReader {
    reader: io::BufReader<File>,
}

impl JsonLinesReader {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        Ok(Self { reader })
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
