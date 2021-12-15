use std::{
    fs::File,
    io::self,
};
use json_streams::json_lines::JsonLinesReader;
use serde_json::json;

#[test]
fn stream_json_from_file() {
    let reader = match JsonLinesReader::open("data/test.jsonl") {
        Err(why) => panic!("error: {}", why),
        Ok(reader) => reader,
    };
    let mut count: i32 = 0;

    for obj in reader {
        assert_eq!(obj, json!("test"));
        count += 1;
    }
    assert_eq!(count, 5);
}

#[test]
fn stream_untyped_persons_from_file() {
    let reader = match JsonLinesReader::open("data/persons.jsonl") {
        Err(why) => panic!("error: {}", why),
        Ok(reader) => reader,
    };
    let mut count: i32 = 0;

    for obj in reader {
        assert_eq!(obj["age"], 43);
        count += 1;
    }
    assert_eq!(count, 2);

}
