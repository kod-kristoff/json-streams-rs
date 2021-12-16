pub mod json_iter;
pub mod json_lines;
mod json_value;
mod parsers;

// pub use json_lines::JsonLinesReader;

pub use json_value::JsonValue;

use std::io;

pub fn load_from_file(
    path: &str
) -> Result<Box<dyn Iterator<Item=JsonValue>>, Error> {
    eprintln!("called load_from_file(path={})", path);
    if path == "-" || path.ends_with(".jsonl") {
        return json_lines::load_from_file(path);
    }
    json_iter::load_from_file(path)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
