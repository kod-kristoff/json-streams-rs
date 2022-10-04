use json_streams;
use serde_json;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Begin");
    for obj in json_streams::json_lines::JsonLinesReader::open("data.jsonl")? {
        println!("{}", obj);
    }
    Ok(())
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
