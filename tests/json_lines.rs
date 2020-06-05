use json_streams::json_lines;

#[test]
fn stream_json_from_file() {
    let mut count: i32 = 0;
    for obj in json_lines::JsonLinesReader::open("test.jsonl") {
        assert_eq!(obj?, "test");
        count += 1;
    }
    assert_eq!(count, 5);
}
