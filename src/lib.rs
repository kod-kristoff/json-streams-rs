mod parsers;
pub mod json_iter;
mod json_value;
pub mod json_lines;

// pub use json_lines::JsonLinesReader;

pub use json_value::JsonValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
