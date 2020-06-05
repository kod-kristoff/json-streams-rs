use std::{
    fs::File,
    io::self,
};
use json_streams::json_lines;

#[test]
fn stream_json_from_file() {
    let file = match File::open("data/test.jsonl") {
        Err(why) => panic!("error: {}", why),
        Ok(file) => file,
    };
    let reader = json_lines::JsonLinesReader {
        reader: io::BufReader::new(file),
        buf: Default::default(),
    };
    let mut count: i32 = 0;

    for obj in reader {
        assert_eq!(*obj, "\"test\"\n");
        count += 1;
    }
    assert_eq!(count, 5);
}
