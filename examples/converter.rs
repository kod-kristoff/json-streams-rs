use serde_json;
use json_streams::json_lines::serde_json_lines::JsonLinesReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Begin");
    for obj in JsonLinesReader::open("data.jsonl")? {
        println!("{}", obj);
    }
    Ok(())
}

