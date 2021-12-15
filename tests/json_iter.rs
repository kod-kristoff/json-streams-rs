use json_streams::{json_iter::load_from_read, JsonValue};
use std::{fs::File, io};

#[test]
fn stream_json_from_file() {
    let file = match File::open("data/test.json") {
        Err(why) => panic!("error: {}", why),
        Ok(file) => file,
    };
    let mut count: i32 = 0;

    for obj in load_from_read(file) {
        assert_eq!(obj, JsonValue::Str("test".to_string()));
        count += 1;
    }
    assert_eq!(count, 5);
}

#[test]
fn stream_untyped_persons_from_file() {
    let file = match File::open("data/persons.json") {
        Err(why) => panic!("error: {}", why),
        Ok(file) => file,
    };
    let mut count: i32 = 0;

    for obj in load_from_read(file) {
        // assert_eq!(obj["age"], 43);
        count += 1;
    }
    assert_eq!(count, 2);
}
