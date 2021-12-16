pub mod nom_json_lines;
pub mod serde_json_lines;

use nom_json_lines::NomJsonLinesIterator;
use crate::{JsonValue, Error};

use std::io;
use std::fs;

pub fn load_from_file(
    path: &str
) -> Result<Box<dyn Iterator<Item=JsonValue>>, Error> {
    eprintln!("called json_lines::load_from_file(path='{}')", path);
    if path == "-" {
        eprintln!("reading from <stdin>");
        let iter = Box::new(
            NomJsonLinesIterator::from_bufreader(
                Box::new(io::BufReader::new(io::stdin()))
            )
        );
        return Ok(iter);
    }
    eprintln!("reading from '{}'", path);
    let iter = Box::new(
        NomJsonLinesIterator::from_bufreader(
            Box::new(io::BufReader::new(fs::File::open(path)?))
        )
    );
    Ok(iter)
}
