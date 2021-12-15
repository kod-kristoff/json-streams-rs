use json_streams::{json_lines::nom_json_lines::NomJsonLinesIterator, JsonValue};
use std::{fs::File, io};

#[test]
fn stream_json_from_file() {
    let reader = match NomJsonLinesIterator::open("data/test.jsonl") {
        Err(why) => panic!("error: {}", why),
        Ok(reader) => reader,
    };
    let mut count: i32 = 0;

    for obj in reader {
        assert_eq!(obj, JsonValue::Str(String::from("test")));
        count += 1;
    }
    assert_eq!(count, 5);
}

#[test]
fn stream_untyped_persons_from_file() {
    let reader = match NomJsonLinesIterator::open("data/persons.jsonl") {
        Err(why) => panic!("error: {}", why),
        Ok(reader) => reader,
    };
    let mut count: i32 = 0;

    for obj in reader {
        match obj {
            JsonValue::Object(obj) => assert_eq!(obj["age"], JsonValue::Num(43.0)),
            _ => panic!(""),
        };
        count += 1;
    }
    assert_eq!(count, 2);
}
